#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

use Color::*;

pub trait ColorAs {
    fn color_as(self, c: Color) -> String;
}

impl ColorAs for String {
    // Surround a string with ASCII control characters to color it
    fn color_as(self, c: Color) -> String {
        format!("\x1b[{}m{}\x1b[0m", code(c), self)
    }
}

// A char can be colored
impl ColorAs for char {
    // Surround a string with ASCII control characters to color it
    fn color_as(self, c: Color) -> String {
        self.to_string().color_as(c)
    }
}

// return the ANSI code for a given color
fn code(c: Color) -> String {
    match c {
        Black => "30",
        Red => "31",
        Green => "32",
        Yellow => "33",
        Blue => "34",
        Magenta => "35",
        Cyan => "36",
        White => "37",
    }
    .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        // uncomment to check if the colors are printed correctly
        // println!("{:+}", "hello".to_string().color_as(Blue));
        // println!("{:+}", "hello".to_string().color_as(Green));
        assert_eq!("hello".to_string().color_as(Black), "\x1b[30mhello\x1b[0m");
        assert_eq!("hello".to_string().color_as(Red), "\x1b[31mhello\x1b[0m");
        assert_eq!("hello".to_string().color_as(Green), "\x1b[32mhello\x1b[0m");
        assert_eq!("hello".to_string().color_as(Yellow), "\x1b[33mhello\x1b[0m");
        assert_eq!("hello".to_string().color_as(Blue), "\x1b[34mhello\x1b[0m");
        assert_eq!(
            "hello".to_string().color_as(Magenta),
            "\x1b[35mhello\x1b[0m"
        );
        assert_eq!("hello".to_string().color_as(Cyan), "\x1b[36mhello\x1b[0m");
        assert_eq!("hello".to_string().color_as(White), "\x1b[37mhello\x1b[0m");
    }
}
