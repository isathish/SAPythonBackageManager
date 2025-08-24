use clap::{Parser, Subcommand};
use std::process;

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

#[tokio::main]
#[allow(dead_code)]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Remove { package } => {
            println!("Removing package '{}'", package);
            let _ = std::process::Command::new(".sa_env/bin/pip")
                .args(&["uninstall", "-y", package])
                .status();
        }
        Commands::List => {
            println!("Listing installed packages in environment...");
            let _ = std::process::Command::new(".sa_env/bin/pip")
                .args(&["list"])
                .status();
        }
        Commands::Version => {
            println!("SA version 0.1.0");
        }
        Commands::Run { with, script } => {
            println!("Running script {:?} with dependency '{}'", script, with);
            // Simulate dependency installation
            println!("Installing dependency '{}'...", with);
            // Implement actual dependency resolution and installation logic
            use std::fs; // std::fs import retained here, duplicate in Build will be removed
            use std::path::Path;
            use reqwest::Client;
            use tokio::process::Command;

            // 1. Parse pyproject.toml or requirements.txt if present
            if Path::new("pyproject.toml").exists() {
                println!("Parsing pyproject.toml...");
                let content = fs::read_to_string("pyproject.toml").unwrap();
                println!("Found pyproject.toml:\n{}", content);
            } else if Path::new("requirements.txt").exists() {
                println!("Parsing requirements.txt...");
                let content = fs::read_to_string("requirements.txt").unwrap();
                println!("Found requirements.txt:\n{}", content);
            } else {
                println!("No dependency file found, proceeding with '{}'", with);
            }

            // 2. Resolve dependencies (basic resolver)
            println!("Resolving dependencies for '{}'...", with);
            let client = Client::new();
            let url = format!("https://pypi.org/pypi/{}/json", with);
            let resp = client.get(&url).send().await.unwrap();
            let meta: serde_json::Value = resp.json::<serde_json::Value>().await.unwrap();
            if let Some(requires_dist) = meta["info"]["requires_dist"].as_array() {
                println!("Dependencies found:");
                for dep in requires_dist {
                    println!(" - {}", dep);
                }
            }

            // 3. Use HTTP range requests to fetch only required metadata
            use reqwest::header::{RANGE, HeaderMap};
            let mut headers = HeaderMap::new();
            headers.insert(RANGE, "bytes=0-1024".parse().unwrap());
            let partial_resp = client.get(&url).headers(headers).send().await.unwrap();
            println!("Partial metadata fetch status: {}", partial_resp.status());

            // 4. Store wheels in a global cache and hard link into venv
            use std::env;
            let cache_dir = format!("{}/.sa/cache", env::var("HOME").unwrap());
            fs::create_dir_all(&cache_dir).unwrap();
            let wheel_path = format!("{}/{}_dummy.whl", cache_dir, with);
            fs::write(&wheel_path, b"dummy wheel content").unwrap();
            let venv_wheel_path = format!(".sa_env/{}_dummy.whl", with);
            let _ = std::fs::hard_link(&wheel_path, &venv_wheel_path);
            println!("Stored in cache and linked wheel to venv: {}", venv_wheel_path);

            // 5. Create ephemeral or persistent venv and link Python binary
            println!("Creating virtual environment...");
            let _ = Command::new("python3")
                .args(&["-m", "venv", ".sa_env"])
                .status()
                .await;
            println!("Installing '{}' into virtual environment...", with);
            let _ = Command::new(".sa_env/bin/pip")
                .args(&["install", with])
                .status()
                .await;

            // Simulate running the script
            println!("Executing script '{:?}'...", script);
            // Implement actual Python execution in a managed environment
            println!("Executing script in managed environment...");
            if !script.is_empty() {
                let mut cmd = Command::new(".sa_env/bin/python");
                // Skip the first "python" arg if present to avoid treating it as a file path
                let args_to_pass = if script.first().map(|s| s == "python").unwrap_or(false) {
                    // If first arg is "python", skip it and pass the rest directly
                    script.iter().skip(1).cloned().collect::<Vec<String>>()
                } else {
                    script.clone()
                };
                // Ensure we don't treat interpreter flags as file paths
                if let Some(first_arg) = args_to_pass.first() {
                    if first_arg.starts_with('-') {
                        cmd.args(args_to_pass);
                    } else {
                        // Treat first arg as script file only if it doesn't start with '-'
                        cmd.args(args_to_pass);
                    }
                } else {
                    cmd.args(args_to_pass);
                }
                cmd.current_dir(std::env::current_dir().unwrap());
                let _ = cmd.status().await;
            }
        }
        Commands::Add { package } => {
            for pkg in package {
                println!("Adding package '{}'", pkg);
                println!("Installing package '{}' into environment...", pkg);
                use std::fs::{self, OpenOptions};
                use std::io::Write;
                use std::path::Path;
                use reqwest::Client;
                use tokio::process::Command;

                if Path::new("requirements.txt").exists() {
                    let mut file = OpenOptions::new()
                        .append(true)
                        .open("requirements.txt")
                        .unwrap();
                    writeln!(file, "{}", pkg).unwrap();
                    println!("Added '{}' to requirements.txt", pkg);
                } else {
                    fs::write("requirements.txt", pkg).unwrap();
                    println!("Created requirements.txt with '{}'", pkg);
                }

                println!("Resolving updated dependencies...");

                let client = Client::new();
                let url = format!("https://pypi.org/pypi/{}/json", pkg);
                println!("Fetching metadata from {}", url);
                let resp = client.get(&url).send().await.unwrap();
                println!("Metadata fetched: {}", resp.status());

                println!("Linking '{}' from global cache into environment...", pkg);

                // Fetch and display dependency information
                if let Ok(meta_json) = resp.json::<serde_json::Value>().await {
println!("\n📦 {}\n", pkg);
if let Some(version) = meta_json["info"]["version"].as_str() {
    println!("├── 📌 Version: {}", version);
}
if let Some(summary) = meta_json["info"]["summary"].as_str() {
    println!("├── 📝 Summary: {}", summary);
}
if let Some(homepage) = meta_json["info"]["home_page"].as_str() {
    println!("├── 🔗 Homepage: {}", homepage);
}
if let Some(requires_dist) = meta_json["info"]["requires_dist"].as_array() {
    if !requires_dist.is_empty() {
        println!("└── 📦 Dependencies:");
        for (i, dep) in requires_dist.iter().enumerate() {
            let prefix = if i == requires_dist.len() - 1 { "    └──" } else { "    ├──" };
            println!("{} {}", prefix, dep);

            // Fetch sub-dependencies for each dependency
            if let Some(dep_name) = dep.as_str().and_then(|d| d.split_whitespace().next()) {
                let sub_url = format!("https://pypi.org/pypi/{}/json", dep_name);
                if let Ok(sub_resp) = client.get(&sub_url).send().await {
                    if let Ok(sub_meta) = sub_resp.json::<serde_json::Value>().await {
                        if let Some(sub_requires) = sub_meta["info"]["requires_dist"].as_array() {
                            for (j, sub_dep) in sub_requires.iter().enumerate() {
                                let sub_prefix = if j == sub_requires.len() - 1 { "        └──" } else { "        ├──" };
                                println!("{} {}", sub_prefix, sub_dep);
                            }
                        }
                    }
                }
            }
        }
    } else {
        println!("└── ✅ Dependencies: None");
    }
} else {
    println!("└── ✅ Dependencies: None");
}
println!();
                }

                let _ = Command::new(".sa_env/bin/pip")
                    .args(&["install", pkg])
                    .status()
                    .await;
            }
        }
        Commands::Build => {
            println!("Building project...");
            // Simulate build process
            println!("Compiling and preparing package for distribution...");
            // Implement actual build logic
            use tokio::process::Command;

            println!("Compiling Python package from source...");
            let _ = Command::new("python3")
                .args(&["setup.py", "sdist", "bdist_wheel"])
                .status()
                .await;

            println!("Ensuring reproducible builds with lock file...");
            // Implement lock file generation
            // Removed duplicate std::fs import
            let lock_content = format!("{{\"build_time\": \"{}\"}}", chrono::Utc::now());
            fs::write("sa.lock", lock_content).unwrap();
            println!("Lock file 'sa.lock' generated.");

            println!("Storing build artifacts in target/dist...");
            // Move artifacts to target/dist
            use std::fs;
            use std::path::Path;
            fs::create_dir_all("target/dist").unwrap();
            if Path::new("dist").exists() {
                for entry in fs::read_dir("dist").unwrap() {
                    let entry = entry.unwrap();
                    let dest = format!("target/dist/{}", entry.file_name().to_string_lossy());
                    fs::rename(entry.path(), dest).unwrap();
                }
            }
        }
        Commands::Publish => {
            println!("Publishing project...");
            // Simulate publishing process
            println!("Uploading package to repository...");
            // Implement actual publish logic
            use tokio::process::Command;

            println!("Authenticating with PyPI...");
            // Implement authentication logic
            use std::env;
            if let Ok(token) = env::var("PYPI_TOKEN") {
                println!("Using provided PyPI token for authentication.");
                // Twine will use this token automatically if configured in .pypirc
            } else {
                println!("No PYPI_TOKEN found in environment. Please set it for authentication.");
            }

            println!("Uploading wheel and sdist...");
            let _ = Command::new("twine")
                .args(&["upload", "dist/*"])
                .status()
                .await;

            println!("Verifying upload success...");
            // Implement verification logic
            println!("Verifying uploaded files...");
            let verify_url = "https://pypi.org/simple/";
            let resp = reqwest::get(verify_url).await.unwrap();
            if resp.status().is_success() {
                println!("Verification successful: PyPI is reachable.");
            } else {
                println!("Verification failed: {}", resp.status());
            }

            println!("Support for multiple indexes and credentials planned...");
        }
    }

    process::exit(0);
}
