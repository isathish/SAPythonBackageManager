use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use chrono::{DateTime, Utc};

/// SA - Super Accelerated Python Package Manager
#[derive(Parser)]
#[command(name = "sa", bin_name = "sa")]
#[command(about = "Super Accelerated Python Package Manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
        /// Use Docker container for isolated execution
        #[arg(long)]
        docker: bool,
        /// Docker image to use (default: python:3.11-slim)
        #[arg(long, default_value = "python:3.11-slim")]
        docker_image: String,
    },
    /// Install a Python package (like pip install) and show dependencies
    Install {
        /// Package name to install
        package: String,
    },

    /// Add a package to the environment
    Add {
        /// Package name(s) to add
        #[arg(num_args = 1.., trailing_var_arg = true, allow_hyphen_values = true)]
        package: Vec<String>,
        /// Skip security scanning
        #[arg(long)]
        skip_security: bool,
        /// Use specific mirror
        #[arg(long)]
        mirror: Option<String>,
        /// Force cache refresh
        #[arg(long)]
        refresh_cache: bool,
    },
    /// Remove a package from the environment
    Remove {
        /// Package name to remove
        #[arg(short, long)]
        package: String,
        /// Clean cache for this package
        #[arg(long)]
        clean_cache: bool,
    },
    /// Uninstall a Python package (like pip uninstall)
    Uninstall {
        /// Package name to uninstall
        package: String,
    },
    /// List installed packages in the environment
    List {
        /// Show dependency tree
        #[arg(long)]
        tree: bool,
        /// Output format (table, json, graph)
        #[arg(long, default_value = "table")]
        format: String,
    },
    /// Build the project
    Build {
        /// Use Docker for building
        #[arg(long)]
        docker: bool,
    },
    /// Publish the project
    Publish,
    /// Show the current SA version
    Version,
    /// Cache management commands
    Cache {
        #[command(subcommand)]
        action: CacheAction,
    },
    /// Security scanning commands
    Security {
        #[command(subcommand)]
        action: SecurityAction,
    },
    /// Mirror configuration commands
    Mirror {
        #[command(subcommand)]
        action: MirrorAction,
    },
    /// Dependency visualization commands
    Visualize {
        /// Package to visualize
        package: String,
        /// Output format (dot, svg, png)
        #[arg(long, default_value = "dot")]
        format: String,
        /// Output file
        #[arg(short, long)]
        output: Option<String>,
        /// Include transitive dependencies
        #[arg(long)]
        transitive: bool,
    },
    /// Docker integration commands
    Docker {
        #[command(subcommand)]
        action: DockerAction,
    },
}

#[derive(Subcommand)]
pub enum CacheAction {
    /// Clear all cached packages
    Clear,
    /// Show cache statistics
    Stats,
    /// Verify cache integrity
    Verify,
    /// Optimize cache storage
    Optimize,
}

#[derive(Subcommand)]
pub enum SecurityAction {
    /// Scan packages for vulnerabilities
    Scan {
        /// Package to scan (all if not specified)
        package: Option<String>,
        /// Output format (table, json)
        #[arg(long, default_value = "table")]
        format: String,
    },
    /// Update vulnerability database
    Update,
    /// Show security policy
    Policy,
}

#[derive(Subcommand)]
pub enum MirrorAction {
    /// Add a new mirror
    Add {
        /// Mirror name
        name: String,
        /// Mirror URL
        url: String,
        /// Set as default
        #[arg(long)]
        default: bool,
    },
    /// Remove a mirror
    Remove {
        /// Mirror name
        name: String,
    },
    /// List configured mirrors
    List,
    /// Test mirror connectivity
    Test {
        /// Mirror name (test all if not specified)
        name: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum DockerAction {
    /// Create a Docker environment
    Create {
        /// Environment name
        name: String,
        /// Base image
        #[arg(long, default_value = "python:3.11-slim")]
        image: String,
        /// Requirements file
        #[arg(short, long)]
        requirements: Option<String>,
    },
    /// List Docker environments
    List,
    /// Remove a Docker environment
    Remove {
        /// Environment name
        name: String,
    },
    /// Execute command in Docker environment
    Exec {
        /// Environment name
        name: String,
        /// Command to execute
        command: Vec<String>,
    },
}

// Data structures for advanced features
#[derive(Serialize, Deserialize, Clone)]
pub struct CachedPackage {
    pub name: String,
    pub version: String,
    pub hash: String,
    pub download_url: String,
    pub cached_at: DateTime<Utc>,
    pub file_path: PathBuf,
    pub metadata: PackageMetadata,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct PackageMetadata {
    pub description: String,
    pub author: String,
    pub license: String,
    pub dependencies: Vec<String>,
    pub keywords: Vec<String>,
    pub home_page: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SecurityVulnerability {
    pub id: String,
    pub package: String,
    pub version_range: String,
    pub severity: String,
    pub description: String,
    pub fixed_version: Option<String>,
    pub published_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Mirror {
    pub name: String,
    pub url: String,
    pub is_default: bool,
    pub last_tested: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SAConfig {
    pub mirrors: Vec<Mirror>,
    pub cache_dir: PathBuf,
    pub security_enabled: bool,
    pub docker_enabled: bool,
    pub default_python_version: String,
}
