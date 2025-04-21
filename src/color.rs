use crossterm::style::{Color, Stylize};

pub trait FixedColorize {
    fn fixed_blue(self) -> String;
    fn fixed_bright_blue(self) -> String;
    fn fixed_green(self) -> String;
}

impl<'a> FixedColorize for &'a str {
    fn fixed_blue(self) -> String {
        self.with(Color::Rgb { r: 0, g: 128, b: 255 }).to_string()
    }
    fn fixed_bright_blue(self) -> String {
        self.with(Color::Rgb { r: 80, g: 200, b: 255 }).to_string()
    }
    fn fixed_green(self) -> String {
        self.with(Color::Rgb { r: 0, g: 200, b: 90 }).to_string()
    }
}

impl FixedColorize for String {
    fn fixed_blue(self) -> String {
        self.with(Color::Rgb { r: 0, g: 128, b: 255 }).to_string()
    }
    fn fixed_bright_blue(self) -> String {
        self.with(Color::Rgb { r: 80, g: 200, b: 255 }).to_string()
    }
    fn fixed_green(self) -> String {
        self.with(Color::Rgb { r: 0, g: 200, b: 90 }).to_string()
    }
}
