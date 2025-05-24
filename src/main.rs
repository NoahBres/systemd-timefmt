use clap::Parser;
use chrono_systemd_time::parse_timestamp_tz;
use chrono::{Local, Utc};
use std::process::exit;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Timestamp specification following systemd.time format
    timespec: String,

    /// Output UTC time instead of local time
    #[clap(long)]
    utc: bool,

    /// Format string for output (default: "%Y-%m-%d %H:%M:%S")
    #[clap(long, default_value = "%Y-%m-%d %H:%M:%S")]
    format: String,
}

fn main() {
    let args = Args::parse();

    if args.utc {
        match parse_timestamp_tz(&args.timespec, Utc) {
            Ok(parsed) => {
                println!("{}", parsed.format(&args.format));
            }
            Err(e) => {
                eprintln!("Error parsing time: {}", e);
                exit(1);
            }
        }
    } else {
        match parse_timestamp_tz(&args.timespec, Local) {
            Ok(parsed) => {
                println!("{}", parsed.format(&args.format));
            }
            Err(e) => {
                eprintln!("Error parsing time: {}", e);
                exit(1);
            }
        }
    }
}