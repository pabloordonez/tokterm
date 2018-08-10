use tokterm_core::drawing::cell::Cell;
use tokterm_core::drawing::color::Color;

#[derive(Hash, Debug, Copy, Clone, PartialEq, Eq)]
pub struct ColorPair {
    pub foreground: Color,
    pub background: Color,
}

impl ColorPair {
    pub fn new(foreground: Color, background: Color) -> ColorPair {
        ColorPair {
            foreground,
            background,
        }
    }

    pub fn from_cell(cell: Cell) -> ColorPair {
        ColorPair {
            foreground: cell.foreground,
            background: cell.background,
        }
    }
}

pub fn color_to_i16(color: Color) -> i16 {
    match color {
        Color::Black => 0,
        Color::DarkRed => 1,
        Color::DarkGreen => 2,
        Color::DarkYellow => 3,
        Color::DarkBlue => 4,
        Color::DarkMagenta => 5,
        Color::DarkCyan => 6,
        Color::Grey => 7,
        Color::DarkGrey => 8,
        Color::Red => 9,
        Color::Green => 10,
        Color::Yellow => 11,
        Color::Blue => 12,
        Color::Magenta => 13,
        Color::Cyan => 14,
        Color::White => 15,
    }
}