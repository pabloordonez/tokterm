use color::color_to_u8;
use std::io::{stdin, stdout, Stdin, Stdout, Write};
use termion::color;
use termion::cursor::{Goto, Hide, Show};
use termion::input::{MouseTerminal};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::terminal_size;
use tokterm_core::drawing::cell_buffer::CellBuffer;
use tokterm_core::drawing::point_2d::Point2d;
use tokterm_core::drawing::size_2d::Size2d;
use tokterm_core::system::terminal::Terminal;
use tokterm_core::system::window::Window;
use tokterm_core::Result;
use termion::clear;

pub struct TermionTerminal {
    stdout: MouseTerminal<RawTerminal<Stdout>>,
    stdin: Stdin,
}

impl TermionTerminal {
    pub fn create() -> Result<TermionTerminal> {
        let raw_terminal = match stdout().into_raw_mode() {
            Ok(raw_terminal) => raw_terminal,
            Err(_) => return Err("Couldn't enter into raw mode."),
        };

        let stdout = MouseTerminal::from(raw_terminal);
        let stdin = stdin();

        Ok(TermionTerminal { stdout, stdin })
    }

    #[inline]
    pub fn get_stdout(&mut self) -> &mut MouseTerminal<RawTerminal<Stdout>> {
        &mut self.stdout
    }

    #[inline]
    pub fn get_stdin(&mut self) -> &mut Stdin {
        &mut self.stdin
    }
}

impl Terminal for TermionTerminal {
    /// Disposes the terminal object-
    fn dispose(&self) -> Result<()> {
        Ok(())
    }

    /// Shows or hides the cursor.
    fn set_cursor_visibility(&mut self, visible: bool) -> Result<()> {
        if visible {
            match self.stdout.write(format!("{}", Show).as_bytes()) {
                Err(_) => return Err("Couldn't show the cursor."),
                _ => ()
            };
        } else {
            match self.stdout.write(format!("{}", Hide).as_bytes())  {
                Err(_) => return Err("Couldn't hide the cursor."),
                _ => ()
            };
        }

        Ok(())
    }

    /// Moves the console cursor to a given position.
    fn set_cursor(&mut self, position: Point2d) -> Result<()> {
        match self.stdout.write(format!("{}", Goto(position.x as u16 + 1, position.y as u16 + 1)).as_bytes()) {
            Err(_) => return Err("Couldn't set the cursor position."),
            _ => ()
        };
        Ok(())
    }

    /// Gets the current console size in character units.
    fn get_console_size(&self) -> Result<Size2d> {
        let term_size = match terminal_size() {
            Ok(size) => size,
            Err(_) => return Err("Couldn't retrieve"),
        };

        Ok(Size2d {
            width: term_size.0 as usize,
            height: term_size.1 as usize,
        })
    }

    /// Gets the character size in pixel units.
    fn get_char_size(&self, window: &Window) -> Result<Size2d> {
        unimplemented!()
    }

    /// Clears the console screen.
    fn clear(&mut self) -> Result<()> {
        match self.stdout.write(format!("{}", clear::All).as_bytes()) {
            Err(_) => return Err("Couldn't clear the screen."),
            _ => ()
        };
        Ok(())
    }

    /// Draws a `CellBuffer` to the screen.
    fn write(&mut self, cell_buffer: &mut CellBuffer) -> Result<()> {
        self.set_cursor(Point2d::empty())?;

        let mut buffer = String::default();

        for cell in cell_buffer.iter() {
            buffer += &format!(
                "{}{}{}",
                color::Bg(color::AnsiValue(color_to_u8(cell.background))),
                color::Fg(color::AnsiValue(color_to_u8(cell.foreground))),
                cell.character
            );
        }

        match self.stdout.write(buffer.as_bytes()) {
            Err(_) => return Err("Couldn't write to the terminal"),
            _ => ()
        };

        match self.stdout.flush() {
            Err(_) => return Err("Couldn't flush the buffer."),
            _ => ()
        };

        Ok(())
    }
}
