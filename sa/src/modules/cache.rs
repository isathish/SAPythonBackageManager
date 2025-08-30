// mod cache

use std::path::{Path, PathBuf};
use std::fs;
use rusqlite::Connection;
use dirs::cache_dir;
use chrono::{DateTime, Utc};
use crate::modules::models::{CachedPackage};
use tokio::process::Command;

// Core cache system implementation
pub struct PackageCache {
    pub cache_dir: PathBuf,
    pub db_conn: Connection,
}

impl PackageCache {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let cache_dir = cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("sa-cache");

        fs::create_dir_all(&cache_dir)?;

        let db_path = cache_dir.join("cache.db");
        let db_conn = Connection::open(db_path)?;

        // Initialize database schema
        db_conn.execute(
            "CREATE TABLE IF NOT EXISTS cached_packages (
                name TEXT,
                version TEXT,
                hash TEXT,
                download_url TEXT,
                cached_at TEXT,
                file_path TEXT,
                metadata TEXT,
                PRIMARY KEY (name, version)
            )",
            [],
        )?;

        Ok(PackageCache { cache_dir, db_conn })
    }

    #[allow(dead_code)]
    pub fn get_package(&self, name: &str, version: &str) -> Option<CachedPackage> {
        let mut stmt = self.db_conn.prepare(
            "SELECT name, version, hash, download_url, cached_at, file_path, metadata
             FROM cached_packages WHERE name = ?1 AND version = ?2"
        ).ok()?;

        let row = stmt.query_row([name, version], |row| {
            let cached_at_str: String = row.get(4)?;
            let metadata_str: String = row.get(6)?;

            Ok(CachedPackage {
                name: row.get(0)?,
                version: row.get(1)?,
                hash: row.get(2)?,
                download_url: row.get(3)?,
                cached_at: DateTime::parse_from_rfc3339(&cached_at_str)
                    .unwrap_or_else(|_| Utc::now().into())
                    .with_timezone(&Utc),
                file_path: PathBuf::from(row.get::<_, String>(5)?),
                metadata: serde_json::from_str(&metadata_str).unwrap_or_default(),
            })
        }).ok()?;

        // Verify file still exists
        if row.file_path.exists() {
            Some(row)
        } else {
            // Clean up stale entry
            let _ = self.remove_package(name, version);
            None
        }
    }

    #[allow(dead_code)]
    pub fn store_package(&self, package: &CachedPackage) -> Result<(), Box<dyn std::error::Error>> {
        let metadata_json = serde_json::to_string(&package.metadata)?;

        self.db_conn.execute(
            "INSERT OR REPLACE INTO cached_packages
             (name, version, hash, download_url, cached_at, file_path, metadata)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                &package.name,
                &package.version,
                &package.hash,
                &package.download_url,
                &package.cached_at.to_rfc3339(),
                package.file_path.to_string_lossy().as_ref(),
                &metadata_json,
            ),
        )?;

        Ok(())
    }

    pub fn remove_package(&self, name: &str, version: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Remove from database
        self.db_conn.execute(
            "DELETE FROM cached_packages WHERE name = ?1 AND version = ?2",
            [name, version],
        )?;

        // Remove file
        let cache_path = self.cache_dir.join(format!("{}-{}.whl", name, version));
        if cache_path.exists() {
            fs::remove_file(cache_path)?;
        }

        Ok(())
    }

    pub fn clear_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Clear database
        self.db_conn.execute("DELETE FROM cached_packages", [])?;

        // Clear cache directory
        if self.cache_dir.exists() {
            for entry in fs::read_dir(&self.cache_dir)? {
                let entry = entry?;
                if entry.path().extension().map_or(false, |ext| ext == "whl") {
                    fs::remove_file(entry.path())?;
                }
            }
        }

        Ok(())
    }

    pub fn get_stats(&self) -> Result<(usize, u64), Box<dyn std::error::Error>> {
        let mut stmt = self.db_conn.prepare("SELECT COUNT(*) FROM cached_packages")?;
        let count: usize = stmt.query_row([], |row| row.get(0))?;

        let mut total_size = 0u64;
        if self.cache_dir.exists() {
            for entry in fs::read_dir(&self.cache_dir)? {
                let entry = entry?;
                if let Ok(metadata) = entry.metadata() {
                    total_size += metadata.len();
                }
            }
        }

        Ok((count, total_size))
    }
}

pub async fn ensure_venv_exists() -> Result<(), Box<dyn std::error::Error>> {
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

pub async fn install_package_with_cache(
    _package: &str,
    _cache: &mut PackageCache,
    _mirror_manager: &crate::modules::mirrors::MirrorManager,
    _security_scanner: &crate::modules::security::SecurityScanner,
    _skip_security: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Add package to requirements.txt
    let req_path = "requirements.txt";
    let mut requirements = std::fs::read_to_string(req_path).unwrap_or_default();
    if !requirements.contains(_package) {
        requirements.push_str(&format!("\n{}", _package));
        std::fs::write(req_path, requirements)?;
    }

    // Ensure virtual environment exists
    crate::modules::cache::ensure_venv_exists().await?;

    // Install the package using pip in .sa_env
    let status = tokio::process::Command::new(".sa_env/bin/pip")
        .args(["install", _package])
        .status()
        .await?;
    if !status.success() {
        return Err(format!("Failed to install package: {}", _package).into());
    }
    Ok(())
}
