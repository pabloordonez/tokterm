use drawing::color::Color;

#[derive(Hash, Debug, Copy, Clone)]
pub struct Cell {
    pub character: char,
    pub background: Color,
    pub foreground: Color,
}

#[allow(dead_code)]
impl Cell {
    pub fn new(character: char, foreground: Color, background: Color) -> Cell {
        Cell {
            character,
            background,
            foreground,
        }
    }

    pub fn default(character: char) -> Cell {
        Cell {
            character,
            background: Color::Black,
            foreground: Color::Grey,
        }
    }
}
