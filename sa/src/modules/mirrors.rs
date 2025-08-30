use std::path::PathBuf;
use std::fs;
use dirs;
use reqwest::Client;
use crate::modules::models::Mirror;

// Mirror management
pub struct MirrorManager {
    pub config_path: PathBuf,
    pub mirrors: Vec<Mirror>,
}

impl MirrorManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("sa");

        fs::create_dir_all(&config_dir)?;
        let config_path = config_dir.join("mirrors.json");

        let mirrors = if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            serde_json::from_str(&content).unwrap_or_else(|_| vec![
                Mirror {
                    name: "pypi".to_string(),
                    url: "https://pypi.org/simple/".to_string(),
                    is_default: true,
                    last_tested: None,
                    is_active: true,
                }
            ])
        } else {
            vec![
                Mirror {
                    name: "pypi".to_string(),
                    url: "https://pypi.org/simple/".to_string(),
                    is_default: true,
                    last_tested: None,
                    is_active: true,
                }
            ]
        };

        Ok(MirrorManager { config_path, mirrors })
    }

    pub fn add_mirror(&mut self, name: String, url: String, set_default: bool) -> Result<(), Box<dyn std::error::Error>> {
        if set_default {
            for mirror in &mut self.mirrors {
                mirror.is_default = false;
            }
        }

        self.mirrors.push(Mirror {
            name,
            url,
            is_default: set_default,
            last_tested: None,
            is_active: true,
        });

        self.save_config()?;
        Ok(())
    }

    pub fn remove_mirror(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.mirrors.retain(|mirror| mirror.name != name);
        self.save_config()?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_default_mirror(&self) -> Option<&Mirror> {
        self.mirrors.iter().find(|mirror| mirror.is_default && mirror.is_active)
    }

    pub async fn test_mirror(&self, name: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let mirror = self.mirrors.iter()
            .find(|m| m.name == name)
            .ok_or("Mirror not found")?;

        let client = Client::new();
        let test_url = format!("{}/pip/", mirror.url);

        match client.head(&test_url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json_content = serde_json::to_string_pretty(&self.mirrors)?;
        fs::write(&self.config_path, json_content)?;
        Ok(())
    }
}
