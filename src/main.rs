use clap::{App, Arg};
use crossterm::Result;
use regex::{Regex, Match};
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
    // let mut t = timer::Timer::new(0, 0, 2);
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
        )
        .get_matches();

    // Parse out time values
    if tm.is_present("time") {
        // Create the hours, minutes and seconds regular expressions we will
        // use to search the time values of.
        let re_h = Regex::new(r"(\d+)h").unwrap();
        let re_m = Regex::new(r"(\d+)m").unwrap();
        let re_s = Regex::new(r"(\d+)s").unwrap();

        let time_vals: Vec<&str> = tm.values_of("time").unwrap_or_default().collect();
        println!("greater than 0");
        println!("{:?}", &time_vals);
        let hours = get_time_arg(&time_vals, re_h);
        let minutes = get_time_arg(&time_vals, re_m);
        let seconds = get_time_arg(&time_vals, re_s);
        println!("{}: {}: {}", hours, minutes, seconds);
        
    }

    let re_h = Regex::new(r"(\d+)h").unwrap();
    let test_text = "10h";
    println!("{}", re_h.is_match(test_text));
    println!(
        "{}",
        re_h.captures(test_text).unwrap().get(1).unwrap().as_str()
    );

    Ok(())
}

// Function for parsing out the matched value.
fn get_time_arg(time_matches: &Vec<&str>, search_re: Regex) -> u64 {
    let val_match = time_matches.iter().find(|&&a| search_re.is_match(a));
    let match_str = match val_match {
        // We know we can unwrap here because of None is not returned we have captured
        // at least 1 group.
        Some(m) => search_re.captures(m).unwrap().get(1).unwrap().as_str(),
        None => "0"
    };

    match match_str.parse::<u64>() {
        Ok(t) => t,
        _ => 0
    }
}
