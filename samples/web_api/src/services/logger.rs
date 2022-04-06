use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub trait Logger {
    fn log(&self, message: String, level: LogLevel);

    fn info(&self, message: String) {
        self.log(message, LogLevel::Info);
    }

    fn warn(&self, message: String) {
        self.log(message, LogLevel::Warn);
    }

    fn error(&self, message: String) {
        self.log(message, LogLevel::Error);
    }

    fn debug(&self, message: String) {
        self.log(message, LogLevel::Debug);
    }
}
