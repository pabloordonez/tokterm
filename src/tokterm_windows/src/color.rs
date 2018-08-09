use tokterm_core::drawing::color::Color;

#[inline]
#[allow(dead_code)]
pub fn get_color_from_u16(color: u16) -> Color {
    match color & 0x0F {
        0 => Color::Black,
        1 => Color::DarkBlue,
        2 => Color::DarkGreen,
        3 => Color::DarkCyan,
        4 => Color::DarkRed,
        5 => Color::DarkMagenta,
        6 => Color::DarkYellow,
        7 => Color::Grey,
        8 => Color::DarkGrey,
        9 => Color::Blue,
        10 => Color::Green,
        11 => Color::Cyan,
        12 => Color::Red,
        13 => Color::Magenta,
        14 => Color::Yellow,
        15 => Color::White,
        _ => Color::Black,
    }
}

#[inline]
#[allow(dead_code)]
pub fn get_u16_from_color(color: Color) -> u16 {
    match color {
        Color::Black => 0,
        Color::DarkBlue => 1,
        Color::DarkGreen => 2,
        Color::DarkCyan => 3,
        Color::DarkRed => 4,
        Color::DarkMagenta => 5,
        Color::DarkYellow => 6,
        Color::Grey => 7,
        Color::DarkGrey => 8,
        Color::Blue => 9,
        Color::Green => 10,
        Color::Cyan => 11,
        Color::Red => 12,
        Color::Magenta => 13,
        Color::Yellow => 14,
        Color::White => 15,
    }
}
