use crossterm::{Result};
use figlet_rs::FIGfont;
use std::io::{stdout};
use std::{thread, time};
use thread::sleep;

use crate::timechunk::TimeChunk;
use crate::timetext::TimeText;

pub struct Timer {
    time: TimeChunk,
}

impl Timer {
    pub fn new(hours: u64, minutes: u64, seconds: u64) -> Self {
        Timer {
            time: TimeChunk::from_hms(hours, minutes, seconds),
        }
    }

    pub fn countdown(&mut self) -> Result<()> {
        let stdo = stdout();

        let font_string = std::include_str!("../resources/colossal.flf");
        let thick_font = FIGfont::from_content(font_string).unwrap();
        let mut timetext = TimeText::new(String::from(""), thick_font);
        self.time.print_timetext(&stdo, &mut timetext)?;

        // Here we are adding on one half second, so that the user
        // can see the starting time.
        sleep(time::Duration::from_secs_f64(0.5));
        let current_time = time::Instant::now();
        let mut seconds_elapsed = 0;
        while self.time.duration >= current_time.elapsed() {
            if current_time.elapsed().as_secs() >= seconds_elapsed {
                seconds_elapsed += 1;
                // We have this if statement as a special check so that
                // we never will end up with a negative duration.
                if self.time.duration >= current_time.elapsed() {
                    let print_time = self.time.duration - current_time.elapsed();
                    TimeChunk::new(print_time).print_timetext(&stdo, &mut timetext)?;
                }
            }
        }

        // Swipe the terminal one last time, and move the cursor to back to the
        // beginning of the terminal window.
        TimeChunk::wipe_terminal(&stdo)?;
        Ok(())
    }
}
