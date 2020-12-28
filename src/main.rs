use crossterm::Result;
use std::time;

mod timechunk;
mod timer;
mod timer_cli;
mod timetext;

fn main() -> Result<()> {
    let mut time_ui = timer_cli::TimeCLI::new();
    time_ui.gen_user_interface();
    if time_ui.non_empty {
        let start = time::Instant::now();
        let mut t = timer::Timer::new(time_ui.hours, time_ui.hours, time_ui.seconds);
        t.countdown()?;

        println!("Real time: {:?}", start.elapsed());
    }
    Ok(())
}
