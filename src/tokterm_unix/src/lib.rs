extern crate ncurses;
extern crate tokterm_core;

use ncurses::*;
use tokterm_core::Result;

pub fn test() -> Result<()> {
    let mut x = 0.0;
    let y = 0;
    let DELAY = 30000;

    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    loop {
        x += 0.0001;
        clear(); // Clear the screen of all previously-printed characters
        mvprintw(y, x as i32, "o"); // Print our "ball" at the current xy position
        refresh();
    }

    endwin();
    Ok(())
}
