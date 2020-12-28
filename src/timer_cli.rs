use crossterm::{Result, style, QueueableCommand};
use std::{time, io::stdout};
use std::io::Write;

use clap::{self, ArgMatches};
use clap::{App, Arg};
use regex::Regex;

use crate::timer;

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

pub struct TimeCLI {
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
}

impl TimeCLI {
    pub fn new() -> Self {
        TimeCLI {
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }

    pub fn run_interface(&mut self) -> Result<()> {
        self.gen_user_interface();

        if self.non_empty() {
            let mut t = timer::Timer::new(self.hours, self.minutes, self.seconds);
            t.countdown()?;
        } else {
            let mut stdo = stdout();
            stdo.queue(style::SetForegroundColor(style::Color::Red))?;
            stdo.write("Error: ".as_bytes())?;
            stdo.queue(style::ResetColor)?;
            stdo.write(
                "Unable to parse inputs, please refer to the help by running 'teaticker --help'."
                    .as_bytes(),
            )?;
            stdo.flush()?;
        }
        Ok(())
    }

    fn gen_user_interface(&mut self) {
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

        // Process time arguments
        self.process_time_args(&tm);

        // Process flag arguments
        self.process_flag_args(&tm);
    }

    fn process_flag_args(&mut self, matches: &ArgMatches) {
        if matches.is_present("hours") {
            self.hours = get_flag_arg(matches, "hours")
        }
        if matches.is_present("minutes") {
            self.minutes = get_flag_arg(matches, "minutes")
        }
        if matches.is_present("seconds") {
            self.seconds = get_flag_arg(matches, "seconds")
        }
    }

    fn process_time_args(&mut self, matches: &ArgMatches) {
        // Parse out time values
        if matches.is_present("time") {
            // Create the hours, minutes and seconds regular expressions we will
            // use to search the time values of.
            let re_h = Regex::new(r"(\d+)h$").unwrap();
            let re_m = Regex::new(r"(\d+)m$").unwrap();
            let re_s = Regex::new(r"(\d+)s$").unwrap();

            let time_vals: Vec<&str> = matches.values_of("time").unwrap_or_default().collect();

            self.hours = get_time_arg(&time_vals, re_h);
            self.minutes = get_time_arg(&time_vals, re_m);
            self.seconds = get_time_arg(&time_vals, re_s);
        }
    }

    pub fn non_empty(&self) -> bool {
        (self.hours + self.minutes + self.seconds) > 0
    }
}

fn get_flag_arg(matches: &ArgMatches, id: &str) -> u64 {
    let arg = matches.value_of(id).unwrap_or_default();
    match arg.parse::<u64>() {
        Ok(t) => t,
        Err(_) => 0,
    }
}

// Function for parsing out the matched value.
fn get_time_arg(time_matches: &Vec<&str>, search_re: Regex) -> u64 {
    let val_match = time_matches.iter().find(|&&a| search_re.is_match(a));
    let arg = match val_match {
        // We know we can unwrap here because of None is not returned we have captured
        // at least 1 group.
        Some(m) => search_re.captures(m).unwrap().get(1).unwrap().as_str(),
        None => "0",
    };

    match arg.parse::<u64>() {
        Ok(t) => t,
        Err(_) => 0,
    }
}
