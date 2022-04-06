use crate::services::logger::LogLevel;
use crate::Logger;

#[derive(Debug, Copy, Clone)]
pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, message: String, level: LogLevel) {
        match level {
            LogLevel::Debug => log::debug!("{}", message),
            LogLevel::Info => log::info!("{}", message),
            LogLevel::Warn => log::warn!("{}", message),
            LogLevel::Error => log::error!("{}", message),
        }
    }
}
