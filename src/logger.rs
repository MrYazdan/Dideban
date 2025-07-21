use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use colored::Colorize;

/// A lightweight console logger for the Dideban application.
///
/// This logger outputs log messages to the console based on the configured log level.
/// It implements the `log::Log` trait to integrate with the `log` crate.
pub struct ConsoleLogger {
    level: LevelFilter,
}

impl ConsoleLogger {
    /// Creates a new `ConsoleLogger` with the specified log level.
    ///
    /// # Arguments
    /// * `level` - The maximum log level to display (e.g., Error, Warn, Info, Debug, Trace).
    pub fn new(level: LevelFilter) -> Self {
        ConsoleLogger { level }
    }

    /// Initializes the logger and sets it as the global logger.
    ///
    /// # Arguments
    /// * `level` - The maximum log level to display.
    ///
    /// # Returns
    /// * `Ok(())` - Logger initialized successfully.
    /// * `Err(SetLoggerError)` - Failed to set the logger.
    pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
        log::set_boxed_logger(Box::new(ConsoleLogger::new(level)))
            .map(|()| log::set_max_level(level))
    }
}

impl log::Log for ConsoleLogger {
    /// Checks if a log message should be displayed based on its level.
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    /// Logs a message to the console.
    ///
    /// Formats the message with timestamp, level, target, and message content.
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // Choose color based on log level
            let level_str = match record.level() {
                Level::Error => "ERROR".red().bold(),
                Level::Warn => "WARN".yellow(),
                Level::Info => "INFO".green(),
                Level::Debug => "DEBUG".blue(),
                Level::Trace => "TRACE".cyan(),
            };

            // ISO-8601 Datetime pattern
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            println!(
                "[{}] {} - {}: {}",
                timestamp,
                level_str,
                record.target(),
                record.args()
            );
        }
    }

    /// Flushes the logger (no-op for console logging).
    fn flush(&self) {}
}