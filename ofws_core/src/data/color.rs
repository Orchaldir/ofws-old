use crate::data::math::interpolation::{lerp, Interpolate};

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

    /// Returns a new gray color
    pub const fn gray(value: u8) -> Color {
        Color {
            r: value,
            g: value,
            b: value,
        }
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

impl Interpolate for Color {
    /// Interpolates linearly with another color.
    ///
    /// ```
    ///# use ofws_core::data::color::Color;
    ///# use ofws_core::data::math::interpolation::Interpolate;
    /// let color0 = Color::new(  0, 25, 120);
    /// let color1 = Color::new(200, 75, 220);
    /// let result = Color::new(100, 50, 170);
    ///
    /// assert_eq!(color0.lerp(&color1, 0.5), result);
    /// ```
    fn lerp(&self, other: &Color, factor: f32) -> Color {
        Color {
            r: lerp(self.r, other.r, factor),
            g: lerp(self.g, other.g, factor),
            b: lerp(self.b, other.b, factor),
        }
    }
}

impl From<Color> for [u8; 3] {
    fn from(color: Color) -> Self {
        [color.r, color.g, color.b]
    }
}

impl From<Color> for [f32; 3] {
    fn from(color: Color) -> Self {
        [
            color.r() as f32 / 255.0,
            color.g() as f32 / 255.0,
            color.b() as f32 / 255.0,
        ]
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
