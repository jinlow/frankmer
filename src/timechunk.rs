use crossterm::{cursor, terminal, terminal::ClearType, QueueableCommand, Result};
use std::cmp;
use std::io::{Stdout, Write};
use std::time;
use time::Duration;

use crate::timetext::TimeText;

// Container to handle the duration and
// printing output to terminal
#[derive(cmp::PartialEq, cmp::Eq, cmp::PartialOrd, cmp::Ord, Copy, Clone)]
pub struct TimeChunk {
    pub duration: Duration,
}

impl TimeChunk {
    pub fn new(duration: Duration) -> Self {
        TimeChunk { duration }
    }
    pub fn from_hms(hours: u64, minutes: u64, seconds: u64) -> Self {
        let all_hours = Duration::from_secs(hours * 3600);
        let all_mins = Duration::from_secs(minutes * 60);
        let all_secs = Duration::from_secs(seconds);
        TimeChunk {
            duration: all_hours + all_mins + all_secs,
        }
    }

    pub fn to_hms_string(&self) -> String {
        let dsecs = self.duration.as_secs();
        let hours = (dsecs / 60) / 60;
        let minutes = (dsecs / 60) % 60;
        let seconds = dsecs % 60;
        format!("{:0>2}h {:0>2}m {:0>2}s", hours, minutes, seconds)
    }

    pub fn print_timetext(&self, mut stdo: &Stdout, timetext: &mut TimeText) -> Result<()> {
        timetext.update_text(self.to_hms_string());
        let term_shape = terminal::size()?;
        let timetext_edge = (term_shape.0.saturating_sub(timetext.text_length()) / 2) as usize;
        let timetext_height = term_shape.1.saturating_sub(timetext.text_height()) / 2;

        stdo.queue(terminal::Clear(ClearType::All))?
            .queue(cursor::MoveTo(0, timetext_height))?;

        stdo.write(timetext.pad_left_right(" ", timetext_edge).as_bytes())?;
        stdo.queue(cursor::MoveTo(0, 0))?.flush()?;
        Ok(())
    }
}
