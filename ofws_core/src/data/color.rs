/// Represents a color with the RGB color model.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/RGB_color_model).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    /// Returns a new color
    pub const fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    /// Returns the red component
    ///
    /// ```
    ///# use ofws_core::data::color::Color;
    /// assert_eq!(Color::new(0, 1, 2).r(), 0);
    /// ```
    pub fn r(&self) -> u8 {
        self.r
    }

    /// Returns the green component
    ///
    /// ```
    ///# use ofws_core::data::color::Color;
    /// assert_eq!(Color::new(0, 1, 2).g(), 1);
    /// ```
    pub fn g(&self) -> u8 {
        self.g
    }

    /// Returns the blue component
    ///
    /// ```
    ///# use ofws_core::data::color::Color;
    /// assert_eq!(Color::new(0, 1, 2).b(), 2);
    /// ```
    pub fn b(&self) -> u8 {
        self.b
    }
}

impl From<Color> for [u8; 3] {
    fn from(color: Color) -> Self {
        [color.r, color.g, color.b]
    }
}

pub const BLACK: Color = Color::new(0, 0, 0);
pub const BLUE: Color = Color::new(0, 0, 255);
pub const CYAN: Color = Color::new(0, 255, 255);
pub const GREEN: Color = Color::new(0, 255, 0);
pub const MAGENTA: Color = Color::new(255, 0, 255);
pub const RED: Color = Color::new(255, 0, 0);
pub const PINK: Color = Color::new(255, 0, 128);
pub const WHITE: Color = Color::new(255, 255, 255);
pub const YELLOW: Color = Color::new(255, 255, 0);
