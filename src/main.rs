use crossterm::Result;
use std::time;

mod timechunk;
mod timer;
mod timetext;

fn main() -> Result<()> {
    let start = time::Instant::now();
    let mut t = timer::Timer::new(1, 0, 10);
    t.countdown()?;

    println!("Real time: {:?}", start.elapsed());
    Ok(())
}
