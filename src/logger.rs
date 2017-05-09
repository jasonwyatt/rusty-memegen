extern crate log;

use log::{LogRecord, LogLevel, LogMetadata, LogLevelFilter, SetLoggerError};

pub struct StdOutLogger {
    level: LogLevel
}

impl log::Log for StdOutLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= *(&self.level) 
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}

pub fn init_info_log() -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Debug);
        Box::new(StdOutLogger {
            level: LogLevel::Info
        })
    })
}