mod debug;
mod iter_rrule;
mod parser_rrule;

use chrono::DateTime;
use chrono_tz::Tz;
use structopt::StructOpt;

const CRASHES_PATH: &str = "rrule-afl-fuzz/out/default/crashes/";

// Example commands:
// ```bash
// RUST_BACKTRACE=1 cargo run --bin rrule-debugger -- --id 0 rrule
// ```

/// RRule debugger program
///
/// This crate is used to debug the RRule crate itself.
#[derive(StructOpt, Debug)]
#[structopt(name = "rrule-debugger")]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Run id
    #[structopt(short, long)]
    id: Option<u8>,

    /// Run all
    #[structopt(short, long)]
    all: bool,

    /// All available subcommand
    #[structopt(subcommand)]
    cmd: Commands,
}

/// All available subcommands for testing.
#[derive(StructOpt, Debug)]
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
    let opts = Opt::from_args();

    let data_files: Vec<Vec<u8>> = if let Some(id) = opts.id {
        vec![read_crash_file(id as u32).unwrap_or_default()]
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
        let path = path.map(|path| path.path()).unwrap();
        if !path.is_file() {
            continue;
        }
        let filename = path.file_name().unwrap().to_str().unwrap();
        if filename.starts_with(&format!("id:{:06},", id)) {
            println!("Reading file {:?}", path.to_str());
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
        let path = path.map(|path| path.path()).unwrap();
        if !path.is_file() {
            continue;
        }
        let filename = path.file_name().unwrap().to_str().unwrap();
        if filename.starts_with("id:") {
            println!("Reading file {:?}", path.to_str());
            list.push(std::fs::read(path).expect("Something went wrong reading the file"));
        }
    }
    list
}

pub fn print_all_datetimes(list: Vec<DateTime<Tz>>) {
    let formater = |dt: &DateTime<Tz>| -> String { format!("    \"{}\",\n", dt.to_rfc3339()) };
    println!(
        "[\n{}]",
        list.iter().map(formater).collect::<Vec<_>>().join(""),
    );
}
