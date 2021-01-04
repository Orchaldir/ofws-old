/// Defines the size of something in 2 dimensions.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Size2d {
    width: u32,
    height: u32,
}

pub const ZERO: Size2d = Size2d {
    width: 0,
    height: 0,
};

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
}
