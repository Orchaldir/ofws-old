use crate::data::map::attribute::Attribute;
use crate::data::size2d::Size2d;
use std::collections::HashMap;

pub mod attribute;

/// Represents a 2d region or world map.
pub struct Map2d {
    size: Size2d,
    attribute_lookup: HashMap<String, usize>,
    attributes: Vec<Attribute>,
}

impl Map2d {
    /// Returns a new map.
    pub fn new(size: Size2d) -> Map2d {
        Map2d {
            size,
            attribute_lookup: HashMap::new(),
            attributes: Vec::new(),
        }
    }

    /// Adds a new [`Attribute`] to the map and resturns its id.
    ///
    /// ```
    ///# use ofws_core::data::map::attribute::Attribute;
    ///# use ofws_core::data::map::Map2d;
    ///# use ofws_core::data::size2d::Size2d;
    /// let mut map = Map2d::new(Size2d::new(2, 3));
    ///
    /// assert_eq!(map.create_attribute("elevation", 42), Some(0));
    /// assert_eq!(map.create_attribute("rainfall", 100), Some(1));
    /// ```
    ///
    /// Fails if the map already contains an [`Attribute`] with the same name.
    ///
    /// ```
    ///# use ofws_core::data::map::attribute::Attribute;
    ///# use ofws_core::data::map::Map2d;
    ///# use ofws_core::data::size2d::Size2d;
    /// let mut map = Map2d::new(Size2d::new(2, 3));
    ///
    /// assert_eq!(map.create_attribute("elevation", 42), Some(0));
    /// assert_eq!(map.create_attribute("elevation", 100), None);
    /// ```
    pub fn create_attribute<S: Into<String>>(&mut self, name: S, default: u8) -> Option<usize> {
        let id = self.attributes.len();
        let attribute = Attribute::new(name, self.size, default);

        if self.attribute_lookup.contains_key(attribute.get_name()) {
            return None;
        }

        self.attribute_lookup
            .insert(attribute.get_name().to_string(), id);
        self.attributes.push(attribute);
        Some(id)
    }

    /// Returns an [`Attribute`] with the matching id.
    ///
    /// ```
    ///# use ofws_core::data::map::attribute::Attribute;
    ///# use ofws_core::data::map::Map2d;
    ///# use ofws_core::data::size2d::Size2d;
    /// let mut map = Map2d::new(Size2d::new(2, 3));
    /// map.create_attribute("elevation", 42);
    /// map.create_attribute("rainfall", 100);
    ///
    /// assert_eq!(map.get_attribute(0).get_name(), "elevation");
    /// assert_eq!(map.get_attribute(1).get_name(), "rainfall");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if there is no matching id.
    ///
    /// ```should_panic
    ///# use ofws_core::data::map::attribute::Attribute;
    ///# use ofws_core::data::map::Map2d;
    ///# use ofws_core::data::size2d::Size2d;
    /// let mut map = Map2d::new(Size2d::new(2, 3));
    ///
    /// map.get_attribute(0);
    /// ```
    pub fn get_attribute(&self, id: usize) -> &Attribute {
        unwrap!(self.attributes.get(id), "Unknown attribute id {}!", id)
    }
}
