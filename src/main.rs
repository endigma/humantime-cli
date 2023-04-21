use clap::Parser;
use humantime::format_duration;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Parser, Debug)]
#[clap(
    author = "endigma",
    version,
    about = "A super simple utility to format the difference between two timestamps."
)]
struct Cli {
    #[clap(short, long)]
    start: Option<String>,

    #[clap(short, long)]
    end: Option<String>,

    #[clap(action, short, long)]
    disable_suffix: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64;

    let start_timestamp: i64 = match args.start {
        Some(s) => s.parse()?,
        None => now,
    };

    let end_timestamp: i64 = match args.end {
        Some(s) => s.parse()?,
        None => now,
    };

    let dur = Duration::from_secs((start_timestamp - end_timestamp).unsigned_abs());
    let mut dur_string = format_duration(dur).to_string();

    if !args.disable_suffix {
        if start_timestamp > end_timestamp {
            dur_string.push_str(" ago");
        } else {
            dur_string.push_str(" from now");
        }
    }

    println!("{}", dur_string);

    Ok(())
}
