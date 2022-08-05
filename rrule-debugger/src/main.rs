#![allow(clippy::cast_possible_truncation, clippy::doc_markdown)]

mod debug;
mod iter_rrule;
mod parser_rrule;
mod simple_logger;

use chrono::DateTime;
use chrono_tz::Tz;
use clap::Parser;
use log::LevelFilter;

const CRASHES_PATH: &str = "rrule-afl-fuzz/out/default/crashes/";

// Example commands:
// ```bash
// RUST_BACKTRACE=1 cargo run --bin rrule-debugger -- --id 0 rrule
// ```

/// RRule debugger program
///
/// This crate is used to debug the RRule crate itself.
#[derive(Parser, Debug)]
#[clap(name = "rrule-debugger")]
struct Opts {
    /// Activate debug mode
    #[clap(short, long)]
    debug: bool,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Run id
    #[clap(short, long)]
    id: Option<u8>,

    /// Run all
    #[clap(short, long)]
    all: bool,

    /// All available subcommand
    #[clap(subcommand)]
    cmd: Commands,
}

/// All available subcommands for testing.
#[derive(Parser, Debug)]
enum Commands {
    /// Check data from parser
    Parser {},
    /// Parse data from raw binary RRule data
    Rrule {},
    /// Used for debugging particular parts of the code,
    /// for example when a test fails.
    Debug {},
}

fn main() {
    // Get command line arguments
    let opts: Opts = Parser::parse();
    // Get log settings
    initialize_logger(&opts);

    let data_files: Vec<Vec<u8>> = if let Some(id) = opts.id {
        vec![read_crash_file(u32::from(id)).expect("Could not find a crash file with the given id")]
    } else if opts.all {
        read_all_crash_file()
    } else {
        vec!["FREQ=HOURLY;INTERVAL=2".as_bytes().to_vec()]
    };

    match opts.cmd {
        Commands::Parser {} => {
            for (i, data) in data_files.iter().enumerate() {
                parser_rrule::from_crash_file(i as u32, data);
            }
        }
        Commands::Rrule {} => {
            for (i, data) in data_files.iter().enumerate() {
                iter_rrule::from_crash_file(i as u32, data);
            }
        }
        Commands::Debug {} => {
            debug::run_debug_function();
        }
    }
}

/// Read data from crash file
fn read_crash_file(id: u32) -> Option<Vec<u8>> {
    let paths = std::fs::read_dir(CRASHES_PATH).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if !path.is_file() {
            continue;
        }
        let filename = path.file_name().unwrap().to_str().unwrap();
        if filename.starts_with(&format!("id:{:06},", id)) {
            println!("Reading file {:?}", path);
            return Some(std::fs::read(path).expect("Something went wrong reading the file"));
        }
    }
    None
}

/// Read data from crash files
fn read_all_crash_file() -> Vec<Vec<u8>> {
    let paths = std::fs::read_dir(CRASHES_PATH).unwrap();
    let mut list = vec![];
    for path in paths {
        let path = path.unwrap().path();
        if !path.is_file() {
            continue;
        }
        let filename = path.file_name().unwrap().to_str().unwrap();
        if filename.starts_with("id:") {
            println!("Reading file {:?}", path);
            list.push(std::fs::read(path).expect("Something went wrong reading the file"));
        }
    }
    list
}

pub fn print_all_datetimes(list: &[DateTime<Tz>]) {
    let formatter = |dt: &DateTime<Tz>| -> String { format!("    \"{}\",\n", dt.to_rfc3339()) };
    println!("[\n{}]", list.iter().map(formatter).collect::<String>(),);
}

/// Setup logger. This will select where to print the log message and how many.
fn initialize_logger(opts: &Opts) {
    let log_filter: LevelFilter = if opts.debug {
        if opts.verbose >= 2 {
            LevelFilter::Trace
        } else {
            LevelFilter::Debug
        }
    } else {
        match opts.verbose {
            0 => LevelFilter::Info,
            1 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    };
    // Setup logger and log level
    log::set_logger(&simple_logger::LOGGER).unwrap();
    log::set_max_level(log_filter);
}
