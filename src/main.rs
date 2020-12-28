use clap::{App, Arg};
use crossterm::Result;
use std::time;

mod timechunk;
mod timer;
mod timetext;

static TIME_HELP: &'static str = "The time used to initialize the timer with.
This must be the time separated by spaces describing the
hours (h), minutes (m), and seconds (s) to start the timer with.
The 'h', 'm', and 's' post fixes can be used on an integer to specify
the hours, minutes or seconds respectively to start the timer with.
If this is not set, at least one of the '--hour', '--minutes' or 
'--seconds' flags must be set. Refer to the help for these options for
more details.
For example passing:
    - 'teaticker 1h 10m 24s': Start the timer with one hour,
                              10 minutes and 24 seconds.
    - 'teaticker 3m': Start the timer with 3 minutes.";

fn main() -> Result<()> {
    // let start = time::Instant::now();
    // let mut t = timer::Timer::new(0, 0, 1);
    // t.countdown()?;

    // println!("Real time: {:?}", start.elapsed());
    let tm = App::new("TeaTicker")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        // .author("James Inlow, james.d.inlow@gmail.com")
        .version("0.1.0")
        .about("A simple timer for the command line.")
        .arg(
            Arg::new("time")
                .multiple(true)
                .value_delimiter(" ")
                .max_values(3)
                .long_about(TIME_HELP)
                .index(1),
        )
        .arg(
            Arg::new("hours")
                .long("hour")
                .short('H')
                .value_name("N_HOURS")
                .about("The number of hours to start the timer with."),
        )
        .arg(
            Arg::new("minutes")
                .long("minutes")
                .short('M')
                .value_name("N_MINUTES")
                .about("The number of minutes to start the timer with."),
        )
        .arg(
            Arg::new("seconds")
                .long("seconds")
                .short('S')
                .value_name("N_SECONDS")
                .about("The number of seconds to start the timer with."),
        ).get_matches();

    let time_vals: Vec<&str> = tm.values_of("time").unwrap_or_default().collect();

    if time_vals.len() > 0 {
        println!("greater than 0");
        println!("{:?}", time_vals);
        return Ok(());
    }

    Ok(())
}
