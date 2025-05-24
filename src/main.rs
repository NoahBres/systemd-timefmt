use clap::Parser;
use chrono_systemd_time::parse_timestamp_tz;
use chrono::{Local, Utc};
use std::process::exit;
use regex::Regex;

const SYSTEMD_TIME_URL: &str = "https://www.freedesktop.org/software/systemd/man/latest/systemd.time.html";

fn long_about() -> String {
    format!(
"Parse systemd.time string specifications and output formatted timestamps.

EXAMPLES:
    systemd-timefmt \"now -1hr\"
    systemd-timefmt \"today +2days\"
    systemd-timefmt \"11min ago\"
    systemd-timefmt \"+3hr\"
    systemd-timefmt \"tomorrow -3h30m\"
    systemd-timefmt \"@1529578800s\"
    systemd-timefmt \"2023-12-25 15:30:00\" --format \"%B %d, %Y at %I:%M %p\"

Supports systemd.time format, including relative times (now, today, yesterday, tomorrow),
absolute timestamps, epoch times, and time arithmetic with units like h, m, s, d, w, M, y.

For more details: {}", SYSTEMD_TIME_URL
    )
}

/// If the input looks like a joined special token and offset (e.g. "now-1hr"), print a hint.
fn print_heuristic_hint(input: &str) {
    // Regex that matches: ^(now|today|yesterday|tomorrow)[+-]
    let re = Regex::new(r"^(now|today|yesterday|tomorrow)[+-]").unwrap();
    if let Some(caps) = re.captures(input) {
        let token      = &caps[1];
        let suggestion = format!("{} {}", token, &input[token.len()..]);

        eprintln!(
            r#"
You wrote `{full}` which uses a special token (`{token}`) with an attached offset.
According to the systemd.time specification, a space is required after `{token}`.

    Try writing: `{suggestion}`

For more details, see {link}.
"#,
            full = input,
            token = token,
            suggestion = suggestion,
            link = SYSTEMD_TIME_URL
        );
    }
}


#[derive(Parser)]
#[clap(
    version, 
    about,
    long_about = &long_about()
)]
struct Args {
    /// Timestamp specification following systemd.time format
    timespec: String,

    /// Output UTC time instead of local time
    #[clap(long)]
    utc: bool,

    /// Format string for output (default: "%Y-%m-%d %H:%M:%S"). Supports strftime formatting: https://docs.rs/chrono/latest/chrono/format/strftime/index.html
    #[clap(long, default_value = "%Y-%m-%d %H:%M:%S")]
    format: String,
}

fn main() {
    let args = Args::parse();

    // Note: need separate parse_timestamp_tz() calls in the if/else block because
    // the timezone field in parse_timestamp_tz is templated and requires distinct calls.
    if args.utc {
        match parse_timestamp_tz(&args.timespec, Utc) {
            Ok(parsed) => {
                println!("{}", format!("{}", parsed.format(&args.format)));
            }
            Err(e) => {
                eprintln!("Error parsing time: {}", e);
                print_heuristic_hint(&args.timespec);
                exit(1);
            }
        }
    } else {
        match parse_timestamp_tz(&args.timespec, Local) {
            Ok(parsed) => {
                println!("{}", format!("{}", parsed.format(&args.format)));
            }
            Err(e) => {
                eprintln!("Error parsing time: {}", e);
                print_heuristic_hint(&args.timespec);
                exit(1);
            }
        }
    }
}