use color::color_to_i16;
use color::ColorPair;
use ncurses::attron;
use ncurses::chtype;
use ncurses::clear;
use ncurses::constants::ERR;
use ncurses::curs_set;
use ncurses::endwin;
use ncurses::getmaxyx;
use ncurses::init_pair;
use ncurses::initscr;
use ncurses::mvwaddch;
use ncurses::refresh;
use ncurses::setlocale;
use ncurses::wmove;
use ncurses::LcCategory;
use ncurses::COLOR_PAIR;
use ncurses::CURSOR_VISIBILITY;
use ncurses::WINDOW;
use std::collections::HashMap;
use tokterm_core::drawing::cell_buffer::CellBuffer;
use tokterm_core::drawing::point_2d::Point2d;
use tokterm_core::drawing::size_2d::Size2d;
use tokterm_core::system::terminal::Terminal;
use tokterm_core::system::window::Window;
use tokterm_core::Result;

pub struct NCursesTerminal {
    window: WINDOW,
}

impl NCursesTerminal {
    pub fn create() -> Result<NCursesTerminal> {
        let window = initscr();
        setlocale(LcCategory::all, "");
        Ok(NCursesTerminal { window })
    }

    #[inline]
    pub fn get_window(&self) -> WINDOW {
        self.window
    }
}

impl Terminal for NCursesTerminal {
    /// Disposes the terminal object-
    fn dispose(&self) -> Result<()> {
        if endwin() == ERR {
            return Err("Couldn't dispose the window.");
        }

        Ok(())
    }

    /// Shows or hides the cursor.
    fn set_cursor_visibility(&mut self, visible: bool) -> Result<()> {
        match curs_set(if visible {
            CURSOR_VISIBILITY::CURSOR_VISIBLE
        } else {
            CURSOR_VISIBILITY::CURSOR_INVISIBLE
        }) {
            None => Err("Couldn't change the cursor visibility."),
            Some(_) => Ok(()),
        }
    }

    /// Moves the console cursor to a given position.
    fn set_cursor(&mut self, position: Point2d) -> Result<()> {
        if wmove(self.window, position.x as i32, position.y as i32) != ERR {
            Ok(())
        } else {
            Err("Couldn't set the cursor position.")
        }
    }

    /// Gets the current console size in character units.
    fn get_console_size(&self) -> Result<Size2d> {
        let mut x = 0;
        let mut y = 0;

        getmaxyx(self.window, &mut y, &mut x);

        Ok(Size2d::new(x as usize, y as usize))
    }

    /// Gets the character size in pixel units.
    fn get_char_size(&self, window: &Window) -> Result<Size2d> {
        unimplemented!()
    }

    /// Clears the console screen.
    fn clear(&mut self) -> Result<()> {
        if clear() != ERR {
            Ok(())
        } else {
            Err("Couldn't clear the screen.")
        }
    }

    /// Draws a `CellBuffer` to the screen.
    fn write(&mut self, cell_buffer: &mut CellBuffer) -> Result<()> {
        let mut colors: HashMap<ColorPair, i16> = HashMap::new();
        let mut pair_index: i16 = 0;
        let mut index: usize = 0;

        for cell in cell_buffer.iter() {
            let color_pair = ColorPair::from_cell(cell);
            let position = match cell_buffer.coordinates_of(index) {
                Some(point) => point,
                None => return Err("The index of the character was outside the bounds of the buffer.")
            };

            if !colors.contains_key(&color_pair) {
                init_pair(
                    pair_index,
                    color_to_i16(color_pair.foreground),
                    color_to_i16(color_pair.background),
                );
                colors.insert(color_pair, pair_index);
                pair_index += 1;
            }

            attron(COLOR_PAIR(*colors.get(&color_pair).unwrap()));
            mvwaddch(
                self.window,
                position.y,
                position.x,
                cell.character as chtype,
            );

            index += 1;
        }

        refresh();
        Ok(())
    }
}
