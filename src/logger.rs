use log::{LevelFilter, Log, Metadata, Record};

/// A simple logger that writes error messages to stderr and other messages to stdout.
#[derive(Debug)]
pub struct Logger {
    pub level: LevelFilter,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            level: LevelFilter::Info,
        }
    }

    pub fn with_level(mut self, level: LevelFilter) -> Self {
        self.level = level;
        self
    }

    pub fn init(self) -> Result<(), log::SetLoggerError> {
        log::set_max_level(self.level);
        log::set_boxed_logger(Box::new(self))
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // Write to stderr if the log level is Error,
            // otherwise write to stdout:
            match record.level() {
                log::Level::Error => eprintln!("{} {}", record.level(), record.args()),
                _ => println!("{} {}", record.level(), record.args()),
            }
        }
    }

    fn flush(&self) {}
}
