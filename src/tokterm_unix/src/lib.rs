extern crate ncurses;
extern crate tokterm_core;

use ncurses::*;
use tokterm_core::Result;

use std::time::{Duration, Instant};

pub mod application;
pub mod color;
pub mod mouse;
pub mod terminal;
pub mod window;

pub fn test() -> Result<()> {
    let mut x = 0.0;
    let y = 0;
    let DELAY = 30000;

    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut fps = 0;
    let mut frames = 0;
    let mut duration = Duration::from_micros(0);

    loop {
        let now = Instant::now();
        frames += 1;

        clear(); // Clear the screen of all previously-printed characters
        mvprintw(y, x as i32, &format!("{}", fps)); // Print our "ball" at the current xy position
        refresh();

        // checks the frames.
        duration += now.elapsed();

        if duration.as_secs() > 1 {
            duration = Duration::from_micros(0);
            fps = frames;
            frames = 0;
        }
    }

    endwin();
    Ok(())
}
