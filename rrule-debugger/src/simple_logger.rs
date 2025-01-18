use log::{Level, Metadata, Record};
pub use yansi::Paint;

/// An instance of the `Logger`.
pub static LOGGER: Logger = Logger;
/// The log collector and handler for the most printed messages in the terminal.
pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let enable = if cfg!(debug_assertions) {
            // Don't apply additional filters in debug build
            true
        } else {
            // Only in release mode
            // Do the filters below unless it is a Warning, Error (or Debug)
            metadata.level() == Level::Warn
                || metadata.level() == Level::Error
                || metadata.level() == Level::Debug
        };

        // All messages need to be Trace or lower
        metadata.level() <= Level::Trace
            // If release mode filter on
            && enable
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // Print to stderr instead of stdout
            eprintln!(
                "{:<5}:{} - {}",
                match record.level() {
                    Level::Error => Paint::red("ERROR"),
                    Level::Warn => Paint::yellow("WARN"),
                    Level::Info => Paint::blue("INFO"),
                    Level::Debug => Paint::green("DEBUG"),
                    Level::Trace => Paint::magenta("TRACE"),
                },
                Paint::new(record.target()).dim(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}
