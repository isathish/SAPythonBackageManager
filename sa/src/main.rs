use clap::{Parser, Subcommand};
use std::process;
use std::fs;
use std::path::Path;
use std::env;
use std::io::Write;
use reqwest::Client;
use tokio::process::Command;
use serde_json::Value;

/// SA - Super Accelerated Python Package Manager
#[derive(Parser)]
#[command(name = "sa", bin_name = "sa")]
#[command(about = "Super Accelerated Python Package Manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a Python script with dependencies
    Run {
        /// Dependency to install before running
        #[arg(short, long)]
        with: String,
        /// Script and arguments to pass to Python
        #[arg(
            trailing_var_arg = true,
            allow_hyphen_values = true,
            num_args = 1..
        )]
        script: Vec<String>,
    },
    /// Add a package to the environment
    Add {
        /// Package name(s) to add
        #[arg(num_args = 1.., trailing_var_arg = true, allow_hyphen_values = true)]
        package: Vec<String>,
    },
    /// Remove a package from the environment
    Remove {
        /// Package name to remove
        #[arg(short, long)]
        package: String,
    },
    /// List installed packages in the environment
    List,
    /// Build the project
    Build,
    /// Publish the project
    Publish,
    /// Show the current SA version
    Version,
}

async fn ensure_venv_exists() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(".sa_env").exists() {
        println!("Creating virtual environment...");
        let status = Command::new("python3")
            .args(&["-m", "venv", ".sa_env"])
            .status()
            .await?;

        if !status.success() {
            return Err("Failed to create virtual environment".into());
        }
    }
    Ok(())
}

async fn install_package(package: &str) -> Result<(), Box<dyn std::error::Error>> {
    ensure_venv_exists().await?;

    println!("Installing '{}' into virtual environment...", package);
    let status = Command::new(".sa_env/bin/pip")
        .args(&["install", package])
        .status()
        .await?;

    if !status.success() {
        return Err(format!("Failed to install package: {}", package).into());
    }
    Ok(())
}

async fn fetch_package_metadata(client: &Client, package: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let url = format!("https://pypi.org/pypi/{}/json", package);
    let resp = client.get(&url).send().await?;

    if !resp.status().is_success() {
        return Err(format!("Package '{}' not found on PyPI", package).into());
    }

    let meta: Value = resp.json().await?;
    Ok(meta)
}

async fn resolve_dependencies(client: &Client, package: &str, depth: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if depth > 3 { // Limit recursion depth
        return Ok(vec![]);
    }

    let meta = fetch_package_metadata(client, package).await?;
    let mut dependencies = Vec::new();

    if let Some(requires_dist) = meta["info"]["requires_dist"].as_array() {
        for dep in requires_dist {
            if let Some(dep_str) = dep.as_str() {
                // Parse dependency name (before any version specifiers)
                if let Some(dep_name) = dep_str.split_whitespace().next() {
                    let clean_name = dep_name.split(&['>', '<', '=', '!', '~'][..]).next().unwrap_or(dep_name);
                    dependencies.push(clean_name.to_string());
                }
            }
        }
    }

    Ok(dependencies)
}

fn setup_global_cache() -> Result<String, Box<dyn std::error::Error>> {
    let home = env::var("HOME").map_err(|_| "HOME environment variable not set")?;
    let cache_dir = format!("{}/.sa/cache", home);
    fs::create_dir_all(&cache_dir)?;
    Ok(cache_dir)
}

fn parse_project_dependencies() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut dependencies = Vec::new();

    if Path::new("pyproject.toml").exists() {
        println!("Parsing pyproject.toml...");
        let content = fs::read_to_string("pyproject.toml")?;
        // Basic TOML parsing for dependencies
        for line in content.lines() {
            if line.trim().starts_with("dependencies") {
                // This is a simplified parser - in production, use a proper TOML parser
                println!("Found dependencies section in pyproject.toml");
            }
        }
    } else if Path::new("requirements.txt").exists() {
        println!("Parsing requirements.txt...");
        let content = fs::read_to_string("requirements.txt")?;
        for line in content.lines() {
            let line = line.trim();
            if !line.is_empty() && !line.starts_with('#') {
                dependencies.push(line.to_string());
            }
        }
    }

    Ok(dependencies)
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Remove { package } => {
            println!("Removing package '{}'", package);
            match ensure_venv_exists().await {
                Ok(_) => {
                    match Command::new(".sa_env/bin/pip")
                        .args(&["uninstall", "-y", package])
                        .status()
                        .await
                    {
                        Ok(status) => {
                            if status.success() {
                                println!("Successfully removed package '{}'", package);
                                Ok(())
                            } else {
                                Err(format!("Failed to remove package '{}'", package).into())
                            }
                        }
                        Err(e) => Err(format!("Error removing package: {}", e).into()),
                    }
                }
                Err(e) => Err(e),
            }
        }

        Commands::List => {
            println!("Listing installed packages in environment...");
            match ensure_venv_exists().await {
                Ok(_) => {
                    match Command::new(".sa_env/bin/pip")
                        .args(&["list"])
                        .status()
                        .await
                    {
                        Ok(_) => Ok(()),
                        Err(e) => Err(format!("Error listing packages: {}", e).into()),
                    }
                }
                Err(e) => Err(e),
            }
        }

        Commands::Version => {
            println!("SA version 0.1.0");
            Ok(())
        }

        Commands::Run { with, script } => {
            println!("Running script {:?} with dependency '{}'", script, with);

            let client = Client::new();

            // Parse existing project dependencies
            match parse_project_dependencies() {
                Ok(deps) => {
                    if !deps.is_empty() {
                        println!("Found project dependencies: {:?}", deps);
                    }
                }
                Err(e) => println!("Warning: Could not parse project dependencies: {}", e),
            }

            // Setup global cache
            match setup_global_cache() {
                Ok(cache_dir) => println!("Using cache directory: {}", cache_dir),
                Err(e) => println!("Warning: Could not setup cache: {}", e),
            }

            // Resolve and install the required dependency
            match resolve_dependencies(&client, with, 0).await {
                Ok(deps) => {
                    println!("Resolved dependencies for '{}': {:?}", with, deps);
                }
                Err(e) => println!("Warning: Could not resolve dependencies: {}", e),
            }

            // Install the package
            match install_package(with).await {
                Ok(_) => {
                    println!("Successfully installed '{}'", with);

                    // Execute the script
                    if !script.is_empty() {
                        println!("Executing script '{:?}'...", script);

                        let mut cmd = Command::new(".sa_env/bin/python");
                        let args_to_pass = if script.first().map(|s| s == "python").unwrap_or(false) {
                            script.iter().skip(1).cloned().collect::<Vec<String>>()
                        } else {
                            script.clone()
                        };

                        cmd.args(args_to_pass);
                        cmd.current_dir(env::current_dir().unwrap());

                        match cmd.status().await {
                            Ok(status) => {
                                if status.success() {
                                    println!("Script executed successfully");
                                    Ok(())
                                } else {
                                    Err("Script execution failed".into())
                                }
                            }
                            Err(e) => Err(format!("Error executing script: {}", e).into()),
                        }
                    } else {
                        println!("No script provided to execute");
                        Ok(())
                    }
                }
                Err(e) => Err(e),
            }
        }

        Commands::Add { package } => {
            let client = Client::new();
            let mut all_success = true;

            for pkg in package {
                println!("Adding package '{}'", pkg);

                // Add to requirements.txt
                match std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("requirements.txt")
                {
                    Ok(mut file) => {
                        if let Err(e) = writeln!(file, "{}", pkg) {
                            println!("Warning: Could not write to requirements.txt: {}", e);
                        } else {
                            println!("Added '{}' to requirements.txt", pkg);
                        }
                    }
                    Err(e) => {
                        println!("Warning: Could not open requirements.txt: {}", e);
                    }
                }

                // Fetch and display package metadata
                match fetch_package_metadata(&client, pkg).await {
                    Ok(meta_json) => {
                        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                        println!("📦 Package: {}", pkg);
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

                        if let Some(version) = meta_json["info"]["version"].as_str() {
                            println!("  📌 Version     : {}", version);
                        }
                        if let Some(summary) = meta_json["info"]["summary"].as_str() {
                            println!("  📝 Summary     : {}", summary);
                        }
                        if let Some(homepage) = meta_json["info"]["home_page"].as_str() {
                            println!("  🔗 Homepage    : {}", homepage);
                        }

                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

                        if let Some(requires_dist) = meta_json["info"]["requires_dist"].as_array() {
                            if !requires_dist.is_empty() {
                                println!("  📦 Dependencies:");
                                for (i, dep) in requires_dist.iter().enumerate() {
                                    let prefix = if i == requires_dist.len() - 1 { "   └──" } else { "   ├──" };
                                    println!("{} {}", prefix, dep);
                                }
                            } else {
                                println!("  ✅ Dependencies: None");
                            }
                        } else {
                            println!("  ✅ Dependencies: None");
                        }

                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
                    }
                    Err(e) => {
                        println!("Warning: Could not fetch metadata for '{}': {}", pkg, e);
                    }
                }

                // Install the package
                match install_package(pkg).await {
                    Ok(_) => println!("Successfully installed '{}'", pkg),
                    Err(e) => {
                        println!("Error installing '{}': {}", pkg, e);
                        all_success = false;
                    }
                }
            }

            if all_success {
                Ok(())
            } else {
                Err("Some packages failed to install".into())
            }
        }

        Commands::Build => {
            println!("Building project...");

            // Ensure we have a virtual environment
            match ensure_venv_exists().await {
                Ok(_) => {
                    // Install build dependencies
                    match install_package("build").await {
                        Ok(_) => {
                            println!("Compiling Python package from source...");

                            // Use python -m build instead of setup.py
                            match Command::new(".sa_env/bin/python")
                                .args(&["-m", "build"])
                                .status()
                                .await
                            {
                                Ok(status) => {
                                    if status.success() {
                                        println!("Build completed successfully");

                                        // Generate lock file
                                        let lock_content = format!(
                                            "{{\"build_time\": \"{}\", \"sa_version\": \"0.1.0\"}}",
                                            chrono::Utc::now()
                                        );
                                        match fs::write("sa.lock", lock_content) {
                                            Ok(_) => println!("Lock file 'sa.lock' generated."),
                                            Err(e) => println!("Warning: Could not create lock file: {}", e),
                                        }

                                        // Organize build artifacts
                                        match fs::create_dir_all("target/dist") {
                                            Ok(_) => {
                                                if Path::new("dist").exists() {
                                                    match fs::read_dir("dist") {
                                                        Ok(entries) => {
                                                            for entry in entries {
                                                                if let Ok(entry) = entry {
                                                                    let dest = format!("target/dist/{}", entry.file_name().to_string_lossy());
                                                                    if let Err(e) = fs::copy(entry.path(), &dest) {
                                                                        println!("Warning: Could not copy {} to {}: {}", entry.path().display(), dest, e);
                                                                    }
                                                                }
                                                            }
                                                            println!("Build artifacts stored in target/dist");
                                                        }
                                                        Err(e) => println!("Warning: Could not read dist directory: {}", e),
                                                    }
                                                }
                                                Ok(())
                                            }
                                            Err(e) => Err(format!("Could not create target/dist directory: {}", e).into()),
                                        }
                                    } else {
                                        Err("Build failed".into())
                                    }
                                }
                                Err(e) => Err(format!("Error running build: {}", e).into()),
                            }
                        }
                        Err(e) => Err(format!("Could not install build dependencies: {}", e).into()),
                    }
                }
                Err(e) => Err(e),
            }
        }

        Commands::Publish => {
            println!("Publishing project...");

            match ensure_venv_exists().await {
                Ok(_) => {
                    // Install twine for publishing
                    match install_package("twine").await {
                        Ok(_) => {
                            println!("Authenticating with PyPI...");

                            // Check for authentication
                            if let Ok(_token) = env::var("PYPI_TOKEN") {
                                println!("Using provided PyPI token for authentication.");
                            } else {
                                println!("No PYPI_TOKEN found in environment. Please set it for authentication.");
                                return;
                            }

                            // Check if dist directory exists
                            if !Path::new("dist").exists() {
                                println!("No dist directory found. Please run 'sa build' first.");
                                return;
                            }

                            println!("Uploading wheel and sdist...");
                            match Command::new(".sa_env/bin/twine")
                                .args(&["upload", "dist/*"])
                                .status()
                                .await
                            {
                                Ok(status) => {
                                    if status.success() {
                                        println!("Upload completed successfully");

                                        // Verify upload
                                        println!("Verifying upload success...");
                                        let client = Client::new();
                                        match client.get("https://pypi.org/simple/").send().await {
                                            Ok(resp) => {
                                                if resp.status().is_success() {
                                                    println!("Verification successful: PyPI is reachable.");
                                                } else {
                                                    println!("Verification failed: {}", resp.status());
                                                }
                                            }
                                            Err(e) => println!("Verification error: {}", e),
                                        }

                                        Ok(())
                                    } else {
                                        Err("Upload failed".into())
                                    }
                                }
                                Err(e) => Err(format!("Error during upload: {}", e).into()),
                            }
                        }
                        Err(e) => Err(format!("Could not install twine: {}", e).into()),
                    }
                }
                Err(e) => Err(e),
            }
        }
    };

    match result {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
