use serde::Deserialize;
use directories::ProjectDirs;
use std::fs;
use std::path::Path;

/// Configuration structure for the Dideban application.
///
/// This struct holds all configuration parameters for the website and service monitoring tool,
/// loaded from a TOML configuration file or default values.
#[derive(Deserialize, Clone)]
pub struct AppConfig {
    /// List of domains to monitor (e.g., ["https://example.com", "https://google.com"]).
    pub domains: Vec<String>,
    /// Interval between monitoring checks, in seconds.
    pub interval: u64,
    /// Host address for the web server (e.g., "0.0.0.0").
    pub server_host: String,
    /// Port for the web server (e.g., 8000).
    pub server_port: u16,
    /// Enable Bale notifications.
    pub enable_bale: bool,
    /// Bale bot token for notifications.
    pub bale_token: String,
    /// Bale chat ID for notifications.
    pub bale_chat_id: String,
    /// Username for web interface authentication.
    pub web_username: String,
    /// Password for web interface authentication.
    pub web_password: String,
    /// Path to the database file (e.g., "dideban.db").
    pub db_path: String,
    /// Logging level (e.g., "error", "warn", "info", "debug", "trace").
    pub log_level: String,
}

impl AppConfig {
    /// Loads configuration from a TOML file.
    ///
    /// In debug mode, reads from `./config.toml` in the project directory.
    /// In release mode, reads from the system configuration directory
    /// (e.g., `/etc/dideban/config.toml` on Linux or `%APPDATA%\dideban\config.toml` on Windows).
    /// If the file does not exist, falls back to default values.
    ///
    /// # Returns
    /// - `Ok(AppConfig)`: Successfully loaded configuration.
    /// - `Err(String)`: Error message if file reading or parsing fails.
    pub fn from_file() -> Result<Self, String> {
        let config_path = if cfg!(debug_assertions) {
            Path::new("config.toml").to_path_buf()
        } else {
            let proj_dirs = ProjectDirs::from("com", "dideban", "dideban")
                .ok_or("Could not determine config directory")?;
            proj_dirs.config_dir().join("config.toml")
        };

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let config_content = fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config file {}: {}", config_path.display(), e))?;

        let config: AppConfig = toml::from_str(&config_content)
            .map_err(|e| format!("Failed to parse config file: {}", e))?;

        // Validate log_level
        if !["error", "warn", "info", "debug", "trace"].contains(&config.log_level.as_str()) {
            return Err(format!("Invalid log_level: {}. Must be one of: error, warn, info, debug, trace", config.log_level));
        }

        // Validate interval
        if config.interval == 0 {
            return Err("interval cannot be zero".to_string());
        }

        // Validate domains
        if config.domains.is_empty() {
            return Err("domains cannot be empty".to_string());
        }

        Ok(config)
    }

    /// Returns default configuration values.
    ///
    /// Used when the configuration file is not found or cannot be parsed.
    fn default() -> Self {
        Self {
            domains: vec![],
            interval: 60,
            server_host: "127.0.0.1".to_string(),
            server_port: 7000,
            enable_bale: false,
            bale_token: String::new(),
            bale_chat_id: String::new(),
            web_username: "admin".to_string(),
            web_password: "admin".to_string(),
            db_path: "dideban.db".to_string(),
            log_level: "info".to_string(),
        }
    }
}