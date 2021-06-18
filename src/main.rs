use chrono::{Duration, Local, TimeZone};
use std::{
    env, fmt,
    io::{self, Write},
    process, thread,
};

/// The format in which dates should be supplied to the program.
/// See [chrono's docs](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html)
const DATE_FORMAT: &'static str = "%Y-%m-%d %T";

fn main() {
    let date = env::args().nth(1).unwrap_or_else(|| {
        println!("Expected a date in the format: {}", DATE_FORMAT);
        process::exit(1);
    });

    let event = Local
        .datetime_from_str(&date, DATE_FORMAT)
        .unwrap_or_else(|err| {
            println!("Error parsing date: {}", err);
            println!("Expecting format: {}", DATE_FORMAT);
            process::exit(1);
        });

    loop {
        // compute time until event as a partitioned duration
        let until = PartitionedDuration::new(event - Local::now());

        // display duration, wait, then erase line
        print!("{}", until);
        io::stdout().flush().unwrap();
        thread::sleep(std::time::Duration::from_millis(500));
        print!("\x1b[2K\x1b[1G"); // erase line with escape codes
        io::stdout().flush().unwrap();
    }
}

/// PartitionedDuration represents a Duration, split into component parts in such
/// a way that the sum of the parts (despite their different units) would
/// equal the whole duration.
struct PartitionedDuration {
    weeks: i64,
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
}

impl PartitionedDuration {
    /// Constructs a new PartitionedDuration from a normal chrono::Duration,
    /// by splitting it into its component parts (weeks, days, hours, ...)
    pub fn new(mut duration: Duration) -> Self {
        // extract weeks
        let weeks = duration.num_weeks();
        duration = duration - Duration::weeks(weeks);

        // extract days
        let days = duration.num_days();
        duration = duration - Duration::days(days);

        // extract hours
        let hours = duration.num_hours();
        duration = duration - Duration::hours(hours);

        // extract minutes
        let minutes = duration.num_minutes();
        duration = duration - Duration::minutes(minutes);

        // extract seconds
        let seconds = duration.num_seconds();

        PartitionedDuration {
            weeks,
            days,
            hours,
            minutes,
            seconds,
        }
    }
}

impl fmt::Display for PartitionedDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn plural(quantity: i64) -> String {
            (if quantity == 1 { "" } else { "s" }).to_string()
        }

        write!(
            f,
            "{} week{}, {} day{}, {} hour{}, {} minute{}, and {} second{}",
            self.weeks,
            plural(self.weeks),
            self.days,
            plural(self.days),
            self.hours,
            plural(self.hours),
            self.minutes,
            plural(self.minutes),
            self.seconds,
            plural(self.seconds)
        )
    }
}
