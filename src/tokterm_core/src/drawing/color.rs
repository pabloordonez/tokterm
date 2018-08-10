use std::vec::Vec;

// TODO: remove allow dead code.
#[allow(dead_code)]
#[derive(Hash, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    DarkGrey,
    DarkRed,
    DarkGreen,
    DarkYellow,
    DarkBlue,
    DarkMagenta,
    DarkCyan,
    Grey,
}

impl Color {
    pub fn to_vec() -> Vec<Color> {
        let mut vec: Vec<Color> = Vec::new();

        vec.push(Color::Black);

        vec.push(Color::Grey);
        vec.push(Color::Red);
        vec.push(Color::Green);
        vec.push(Color::Yellow);
        vec.push(Color::Blue);
        vec.push(Color::Magenta);
        vec.push(Color::Cyan);

        vec.push(Color::DarkGrey);
        vec.push(Color::DarkRed);
        vec.push(Color::DarkGreen);
        vec.push(Color::DarkYellow);
        vec.push(Color::DarkBlue);
        vec.push(Color::DarkMagenta);
        vec.push(Color::DarkCyan);


        vec.push(Color::White);
        vec
    }
}
