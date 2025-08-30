use std::path::PathBuf;
use std::fs;
use dirs::cache_dir;
use reqwest::Client;
use serde_json::Value;
use colored::*;
use crate::modules::models::SecurityVulnerability;

// Security scanner implementation
pub struct SecurityScanner {
    pub vulnerability_db: Vec<SecurityVulnerability>,
    pub db_path: PathBuf,
}

impl SecurityScanner {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let cache_dir = cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("sa-cache");

        let db_path = cache_dir.join("vulnerabilities.json");

        let vulnerability_db = if db_path.exists() {
            let content = fs::read_to_string(&db_path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Vec::new()
        };

        Ok(SecurityScanner { vulnerability_db, db_path })
    }

    pub async fn update_vulnerability_db(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", "🔄 Updating vulnerability database...".yellow());

        let client = Client::new();

        // Fetch from PyUp.io safety database (mock implementation)
        let response = client
            .get("https://raw.githubusercontent.com/pyupio/safety-db/master/data/insecure_full.json")
            .send()
            .await?;

        if response.status().is_success() {
            let vulnerabilities_data: Value = response.json().await?;

            // Parse and convert to our format (simplified)
            let mut vulnerabilities = Vec::new();

            if let Some(packages) = vulnerabilities_data.as_object() {
                for (package_name, vulns) in packages {
                    if let Some(vuln_array) = vulns.as_array() {
                        for vuln in vuln_array {
                            if let Some(vuln_obj) = vuln.as_object() {
                                let vulnerability = SecurityVulnerability {
                                    id: vuln_obj.get("id")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("unknown")
                                        .to_string(),
                                    package: package_name.clone(),
                                    version_range: vuln_obj.get("specs")
                                        .and_then(|v| v.as_array())
                                        .and_then(|arr| arr.first())
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("*")
                                        .to_string(),
                                    severity: "medium".to_string(),
                                    description: vuln_obj.get("advisory")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("No description available")
                                        .to_string(),
                                    fixed_version: None,
                                    published_at: chrono::Utc::now(),
                                };
                                vulnerabilities.push(vulnerability);
                            }
                        }
                    }
                }
            }

            self.vulnerability_db = vulnerabilities;

            // Save to local database
            let json_content = serde_json::to_string_pretty(&self.vulnerability_db)?;
            fs::write(&self.db_path, json_content)?;

            println!("{}", "✅ Vulnerability database updated successfully".green());
        }

        Ok(())
    }

    pub fn scan_package(&self, package_name: &str, version: &str) -> Vec<SecurityVulnerability> {
        self.vulnerability_db
            .iter()
            .filter(|vuln| {
                vuln.package == package_name && self.version_matches(version, &vuln.version_range)
            })
            .cloned()
            .collect()
    }

    fn version_matches(&self, version: &str, range: &str) -> bool {
        // Simplified version matching - in production use semver crate
        if range == "*" {
            return true;
        }

        if range.starts_with(">=") {
            let range_version = &range[2..];
            version >= range_version
        } else if range.starts_with("<=") {
            let range_version = &range[2..];
            version <= range_version
        } else if range.starts_with('<') {
            let range_version = &range[1..];
            version < range_version
        } else if range.starts_with('>') {
            let range_version = &range[1..];
            version > range_version
        } else {
            version == range
        }
    }
}
