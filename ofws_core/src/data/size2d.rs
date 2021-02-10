use std::ops::{Add, Mul};

#[svgbobdoc::transform]
/// Defines the size of something (e.g. a map) in 2 dimensions.
///
/// # Diagram
///
/// ```svgbob
///       0   1
///   +----------> x-axis
///   |
///   | +---+---+
/// 0 | | 0 | 1 |
///   | +---+---+
/// 1 | | 2 | 3 |
///   | +---+---+
/// 2 | | 4 | 5 |
///   | +---+---+
///   v
/// y-axis
/// ```
///
/// A size with width 2 & height 3.
/// The numbers are indices of each cell.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Size2d {
    width: u32,
    height: u32,
}

impl Size2d {
    /// Creates a new Size2d
    pub fn new(width: u32, height: u32) -> Size2d {
        Size2d { width, height }
    }

    /// Returns the area covered by this size
    ///
    /// ```
    ///# use ofws_core::data::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.get_area(), 6);
    /// ```
    pub fn get_area(&self) -> usize {
        (self.width * self.height) as usize
    }

    /// Returns the size along the x-axis
    ///
    /// ```
    ///# use ofws_core::data::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.width(), 2);
    /// ```
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the size along the y-axis
    ///
    /// ```
    ///# use ofws_core::data::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.height(), 3);
    /// ```
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Converts an index to the x-coordinate of the equivalent point
    ///
    /// ```
    ///# use ofws_core::data::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.to_x(5), 1);
    /// ```
    pub fn to_x(&self, index: usize) -> u32 {
        index as u32 % self.width
    }

    /// Converts an index to the y-coordinate of the equivalent point
    ///
    /// ```
    ///# use ofws_core::data::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.to_y(5), 2);
    /// ```
    pub fn to_y(&self, index: usize) -> u32 {
        index as u32 / self.width
    }

    /// Converts an index to the equivalent point
    ///
    /// ```
    ///# use ofws_core::data::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.to_x_and_y(5), [1,2]);
    /// ```
    pub fn to_x_and_y(&self, index: usize) -> [u32; 2] {
        [self.to_x(index), self.to_y(index)]
    }

    /// Converts a point to the equivalent index
    ///
    /// ```
    ///# use ofws_core::data::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.to_index(1, 2), 5);
    /// ```
    pub fn to_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    /// Converts a point to the equivalent index.
    ///
    /// ```
    ///# use ofws_core::data::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.saturating_to_index(1, 2), 5);
    /// ```
    ///
    /// Coordinates outside the map are limited to width & height.
    ///
    /// ```
    ///# use ofws_core::data::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    ///
    /// assert_eq!(size.saturating_to_index(2, 2), 5);
    /// assert_eq!(size.saturating_to_index(3, 2), 5);
    /// assert_eq!(size.saturating_to_index(0, 3), 4);
    /// assert_eq!(size.saturating_to_index(0, 4), 4);
    /// ```
    pub fn saturating_to_index(&self, x: u32, y: u32) -> usize {
        let x = x.min(self.width - 1);
        let y = y.min(self.height - 1);
        (y * self.width + x) as usize
    }
}

// Adds 2 sizes
///
/// ```
///# use ofws_core::data::size2d::Size2d;
/// let a = Size2d::new(2, 3);
/// let b = Size2d::new(10, 40);
/// assert_eq!(a + b, Size2d::new(12, 43));
/// ```
impl Add for Size2d {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Size2d {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}

/// Multiplies 2 sizes
///
/// ```
///# use ofws_core::data::size2d::Size2d;
/// let a = Size2d::new(2, 3);
/// let b = Size2d::new(10, 40);
/// assert_eq!(a * b, Size2d::new(20, 120));
/// ```
impl Mul for Size2d {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Size2d {
            width: self.width * other.width,
            height: self.height * other.height,
        }
    }
}