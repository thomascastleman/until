use chrono::{Local, TimeZone};
use duration_breakdown::DurationBreakdown;
use std::{
    env,
    error::Error,
    io::{self, Write},
    thread,
};

/// The format in which dates should be supplied to the program.
/// See [chrono's docs](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html)
const DATE_FORMAT: &str = "%Y-%m-%d %T";

/// Prints a message (without a linebreak) and then flushes stdout immediately.
macro_rules! print_and_flush {
    ($fmt:expr, $($args:tt)*) => {
        print!($fmt, $($args)*);
        io::stdout().flush().unwrap();
    };
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {}", e);
        std::process::exit(-1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let date = env::args()
        .nth(1)
        .ok_or(format!("expected a date in the format: {}", DATE_FORMAT))?;

    let event = Local.datetime_from_str(&date, DATE_FORMAT)?;

    loop {
        // compute time until event as a duration breakdown
        let duration = (event - Local::now()).to_std()?;
        let until = DurationBreakdown::from(duration);

        // display duration, wait, then erase line
        print_and_flush!("{}", until.as_string_hide_zeros());
        thread::sleep(std::time::Duration::from_millis(500));
        print_and_flush!("{}", "\x1b[2K\x1b[1G"); // erase line
    }
}
