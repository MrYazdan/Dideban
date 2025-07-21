use actix_web::{App, HttpServer};
use log::{debug, info, LevelFilter};

mod config;
mod logger;
use config::AppConfig;
use logger::ConsoleLogger;

/// Main entry point for the Dideban application.
///
/// Initializes the custom console logger based on the configured log level,
/// loads the configuration, logs configuration details in debug mode,
/// and starts the Actix Web server without routes.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config = AppConfig::from_file().expect("Failed to load config");

    // Initialize logger with configured log level
    let log_level = match config.log_level.as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info, // Fallback to Info if invalid
    };
    ConsoleLogger::init(log_level).expect("Failed to initialize logger");

    // Log configuration details in debug mode
    if config.log_level == "debug" {
        debug!("Configuration loaded:");
        debug!("  - domains: {:?}", config.domains);
        debug!("  - interval: {} seconds", config.interval);
        debug!("  - server_host: {}", config.server_host);
        debug!("  - server_port: {}", config.server_port);
        debug!("  - enable_bale: {}", config.enable_bale);
        debug!("  - bale_token: {}", config.bale_token);
        debug!("  - bale_chat_id: {}", config.bale_chat_id);
        debug!("  - web_username: {}", config.web_username);
        debug!("  - web_password: [hidden]");
        debug!("  - db_path: {}", config.db_path);
        debug!("  - log_level: {}", config.log_level);
    }

    // Prepare server address
    let bind_address = format!("{}:{}", config.server_host, config.server_port);

    // Log server start message
    info!("🚀 Server running at http://{}/", bind_address);

    // Start Actix Web server without routes
    HttpServer::new(|| App::new())
        .workers(1)
        .bind(&bind_address)?
        .run()
        .await
}
