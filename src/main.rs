use chrono::{Local, Utc};
use chrono_systemd_time::parse_timestamp_tz;
use clap::Parser;
use regex::Regex;
use std::process;

const SYSTEMD_TIME_URL: &str =
    "https://www.freedesktop.org/software/systemd/man/latest/systemd.time.html";

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

For more details: {}",
        SYSTEMD_TIME_URL
    )
}

/// If the input looks like a joined special token and offset (e.g. "now-1hr"), print a hint.
fn print_heuristic_hint(input: &str) {
    let re = Regex::new(r"^(now|today|yesterday|tomorrow)[+-]").expect("invalid regex");
    if let Some(caps) = re.captures(input) {
        let token = &caps[1];
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

/// Generic function to handle time parsing for any timezone
fn handle_time_parsing<Tz: chrono::TimeZone>(
    timespec: &str,
    format: &str,
    timezone: Tz,
) -> Result<(), Box<dyn std::error::Error>>
where
    Tz::Offset: std::fmt::Display,
{
    match parse_timestamp_tz(timespec, timezone) {
        Ok(parsed) => {
            println!("{}", parsed.format(format));
            Ok(())
        }
        Err(e) => {
            eprintln!("Error parsing time: {}", e);
            print_heuristic_hint(timespec);
            Err(e.into())
        }
    }
}

fn main() {
    let args = Args::parse();

    let result = if args.utc {
        handle_time_parsing(&args.timespec, &args.format, Utc)
    } else {
        handle_time_parsing(&args.timespec, &args.format, Local)
    };

    if result.is_err() {
        process::exit(1);
    }
}
