use crossterm::Result;

mod timechunk;
mod timer;
mod timer_cli;
mod timetext;

fn main() -> Result<()> {
    let mut time_ui = timer_cli::TimeCLI::new();
    time_ui.run_interface()?;
    Ok(())
}
