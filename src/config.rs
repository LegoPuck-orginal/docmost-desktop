use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const APP_DIR_NAME: &str = "DocmostApp";
const CONFIG_FILE_NAME: &str = "config.json";
pub const DEFAULT_DOCMOST_URL: &str = "https://doc.home.klehausen.de";
pub const DEFAULT_GITHUB_REPO: &str = "LegoPuck-orginal/docmost-desktop";

fn default_true() -> bool {
    true
}

fn default_zoom() -> f64 {
    1.0
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ServerEntry {
    pub id: String,
    pub name: String,
    pub url: String,
}

fn default_servers() -> Vec<ServerEntry> {
    vec![ServerEntry {
        id: "default".to_string(),
        name: "Home Server".to_string(),
        url: DEFAULT_DOCMOST_URL.to_string(),
    }]
}

fn default_active_server_id() -> Option<String> {
    Some("default".to_string())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub server_url: String,
    pub check_updates_on_startup: bool,
    pub github_repo: String,
    pub clean_dark_theme: bool,
    pub window_width: u32,
    pub window_height: u32,
    pub window_maximized: bool,
    #[serde(default = "default_true")]
    pub excalidraw_pen_mode: bool,
    #[serde(default = "default_true")]
    pub excalidraw_palm_rejection: bool,
    #[serde(default = "default_true")]
    pub excalidraw_dark_theme: bool,
    #[serde(default = "default_true")]
    pub excalidraw_quick_toolbar: bool,
    #[serde(default = "default_zoom")]
    pub ui_zoom_level: f64,
    #[serde(default = "default_servers")]
    pub servers: Vec<ServerEntry>,
    #[serde(default = "default_active_server_id")]
    pub active_server_id: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_url: DEFAULT_DOCMOST_URL.to_string(),
            check_updates_on_startup: true,
            github_repo: DEFAULT_GITHUB_REPO.to_string(),
            clean_dark_theme: true,
            window_width: 1280,
            window_height: 840,
            window_maximized: false,
            excalidraw_pen_mode: true,
            excalidraw_palm_rejection: true,
            excalidraw_dark_theme: true,
            excalidraw_quick_toolbar: true,
            ui_zoom_level: 1.0,
            servers: default_servers(),
            active_server_id: default_active_server_id(),
        }
    }
}

impl AppConfig {
    pub fn config_path() -> PathBuf {
        let base_dir = dirs::config_dir()
            .unwrap_or_else(|| dirs::data_local_dir().unwrap_or_else(|| PathBuf::from(".")));
        base_dir.join(APP_DIR_NAME).join(CONFIG_FILE_NAME)
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(mut config) = serde_json::from_str::<AppConfig>(&content) {
                    if config.servers.is_empty() {
                        let name = if config.server_url.contains("klehausen") {
                            "Home Server".to_string()
                        } else {
                            "Standard Server".to_string()
                        };
                        config.servers.push(ServerEntry {
                            id: "default".to_string(),
                            name,
                            url: config.server_url.clone(),
                        });
                        config.active_server_id = Some("default".to_string());
                    }
                    return config;
                }
            }
        }
        let default_config = Self::default();
        let _ = default_config.save();
        default_config
    }

    pub fn active_server(&self) -> Option<&ServerEntry> {
        if let Some(ref id) = self.active_server_id {
            self.servers.iter().find(|s| s.id == *id)
        } else {
            self.servers.first()
        }
    }

    pub fn set_active_server(&mut self, server_id: &str) -> bool {
        if let Some(found) = self.servers.iter().find(|s| s.id == server_id) {
            self.server_url = found.url.clone();
            self.active_server_id = Some(server_id.to_string());
            true
        } else {
            false
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.server_url, "https://doc.home.klehausen.de");
        assert!(config.check_updates_on_startup);
        assert!(config.excalidraw_pen_mode);
        assert!(config.excalidraw_palm_rejection);
        assert_eq!(config.ui_zoom_level, 1.0);
        assert_eq!(config.window_width, 1280);
        assert_eq!(config.window_height, 840);
    }

    #[test]
    fn test_server_switching() {
        let mut config = AppConfig::default();
        assert_eq!(config.servers.len(), 1);
        assert_eq!(config.active_server().unwrap().name, "Home Server");

        config.servers.push(ServerEntry {
            id: "work".to_string(),
            name: "Work Server".to_string(),
            url: "https://doc.work.com".to_string(),
        });

        assert!(config.set_active_server("work"));
        assert_eq!(config.server_url, "https://doc.work.com");
        assert_eq!(config.active_server().unwrap().id, "work");
    }
}
