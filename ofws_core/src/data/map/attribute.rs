use crate::data::size2d::Size2d;

/// Represents a value with a specific meaning for each cell of a map.
///
/// Examples:
/// * elevation
/// * rainfall
/// * temperature
#[derive(Debug)]
pub struct Attribute {
    name: String,
    size: Size2d,
    values: Vec<u8>,
}

impl Attribute {
    /// Returns a new attribute.
    pub fn new<S: Into<String>>(name: S, size: Size2d, default: u8) -> Attribute {
        let values = vec![default; size.get_area()];
        Attribute {
            name: name.into(),
            size,
            values,
        }
    }

    /// Returns the name of the attribute.
    ///
    /// ```
    ///# use ofws_core::data::map::attribute::Attribute;
    ///# use ofws_core::data::size2d::Size2d;
    /// let attribute = Attribute::new("elevation", Size2d::new(2, 3), 42);
    ///
    /// assert_eq!(attribute.get_name(), "elevation");
    /// ```
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns the size of the map.
    ///
    /// ```
    ///# use ofws_core::data::map::attribute::Attribute;
    ///# use ofws_core::data::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// let attribute = Attribute::new("elevation", size, 42);
    ///
    /// assert_eq!(attribute.get_size(), &size);
    /// ```
    pub fn get_size(&self) -> &Size2d {
        &self.size
    }

    /// Returns the value at the index.
    ///
    /// ```
    ///# use ofws_core::data::map::attribute::Attribute;
    ///# use ofws_core::data::size2d::Size2d;
    /// let attribute = Attribute::new("elevation", Size2d::new(1, 2), 42);
    ///
    /// assert_eq!(attribute.get(0), 42);
    /// assert_eq!(attribute.get(1), 42);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the index is outside the map.
    ///
    /// ```should_panic
    ///# use ofws_core::data::map::attribute::Attribute;
    ///# use ofws_core::data::size2d::Size2d;
    /// let attribute = Attribute::new("elevation", Size2d::new(1, 2), 42);
    ///
    /// attribute.get(2);
    /// ```
    pub fn get(&self, index: usize) -> u8 {
        self.values[index]
    }

    /// Returns the mutable value at the index.
    ///
    /// ```
    ///# use ofws_core::data::map::attribute::Attribute;
    ///# use ofws_core::data::size2d::Size2d;
    /// let mut attribute = Attribute::new("elevation", Size2d::new(1, 2), 42);
    ///
    /// *attribute.get_mut(0) += 4;
    ///
    /// assert_eq!(attribute.get(0), 46);
    /// assert_eq!(attribute.get(1), 42);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the index is outside the map.
    ///
    /// ```should_panic
    ///# use ofws_core::data::map::attribute::Attribute;
    ///# use ofws_core::data::size2d::Size2d;
    /// let mut attribute = Attribute::new("elevation", Size2d::new(1, 2), 42);
    ///
    /// attribute.get_mut(2);
    /// ```
    pub fn get_mut(&mut self, index: usize) -> &mut u8 {
        self.values.get_mut(index).expect("Index is outside map!")
    }
}
