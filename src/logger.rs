/// A simple logger that writes error messages to stderr and other messages to stdout.
use log::{LevelFilter, Log, Metadata, Record};

#[derive(Debug)]
pub struct Logger {
    pub level: LevelFilter,
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
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
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record<'_>) {
        if self.enabled(record.metadata()) {
            eprintln!("{} {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub mod helpers {
    use console::{style, Emoji};
    use log::{error, info, warn};
    pub static SEARCH: Emoji<'_, '_> = Emoji("🔍", "");
    pub static FOUND: Emoji<'_, '_> = Emoji("✨", "");
    pub static INFO: Emoji<'_, '_> = Emoji("ℹ️", "");
    pub static WARN: Emoji<'_, '_> = Emoji("⚠️", "");
    pub static TRASH: Emoji<'_, '_> = Emoji("🗑️", "");
    pub static DOWNLOAD: Emoji<'_, '_> = Emoji("🚚", "");
    pub static VALID: Emoji<'_, '_> = Emoji("✅", "");
    pub static INVALID: Emoji<'_, '_> = Emoji("❌", "");
    pub static ERROR: Emoji<'_, '_> = Emoji("🚨", "/!\\");

    pub fn log_step(emoji: Emoji<'_, '_>, msg: &str) {
        info!("{} {}", emoji, msg);
    }

    pub fn log_info(msg: &str) {
        info!("{} {}", INFO, msg);
    }

    pub fn log_err(msg: &str) {
        error!("{} {}", ERROR, style(msg).bold().red());
    }

    pub fn log_warn(msg: &str) {
        warn!("{} {}", WARN, style(msg).bold().yellow());
    }
}
