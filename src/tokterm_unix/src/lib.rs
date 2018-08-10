extern crate ncurses;
extern crate tokterm_core;

use ncurses::*;
use tokterm_core::Result;

use std::time::{Duration, Instant};
use tokterm_core::drawing::cell_buffer::CellBuffer;
use tokterm_core::drawing::color::Color;
use terminal::UnixTerminal;
use tokterm_core::drawing::point_2d::Point2d;
use tokterm_core::system::terminal::Terminal;
use tokterm_core::drawing::cell::Cell;

pub mod color;
pub mod mouse;
pub mod terminal;
pub mod window;
pub mod application;

pub fn test() -> Result<()> {

    let mut fps = 0;
    let mut frames = 0;
    let mut duration = Duration::from_micros(0);

    let terminal = UnixTerminal::create()?;
    terminal.set_cursor_visibility(false);
    let mut buffer = CellBuffer::new(Cell::new('~', Color::Blue, Color::DarkBlue), terminal.get_console_size()?);

    loop {
        let now = Instant::now();
        frames += 1;

        buffer.resize(Cell::new('~', Color::Blue, Color::DarkBlue), terminal.get_console_size()?);

        buffer.write_str(&format!("FPS: {}", fps), Point2d::new(0, 0), Color::Red, Color::DarkBlue);

        let colors = Color::to_vec();

        for f in 0..16 {
            for b in 0..16 {
                buffer.set(Point2d::new(f + 1, b + 1), Cell::new('@', *colors.get(f).unwrap(), *colors.get(b).unwrap()));
            }
        }

        terminal.write(&buffer);

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
