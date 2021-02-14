use crate::data::map::Map2d;
use serde::{Deserialize, Serialize};

/// Create a new [`Attribute`] of the map.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CreateAttribute {
    name: String,
    default: u8,
}

impl CreateAttribute {
    pub fn new<S: Into<String>>(name: S, default: u8) -> CreateAttribute {
        CreateAttribute {
            name: name.into(),
            default,
        }
    }

    /// Runs the step.
    ///
    /// ```
    ///# use ofws_core::data::map::Map2d;
    ///# use ofws_core::data::map::generation::attributes::create::CreateAttribute;
    ///# use ofws_core::data::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// let mut map = Map2d::new(size);
    /// let step = CreateAttribute::new("test0", 9);
    ///
    /// step.run(&mut map);
    ///
    /// let attribute = map.get_attribute(0);
    /// assert_eq!(attribute.get_name(), "test0");
    /// assert_eq!(attribute.get_size(), &size);
    /// assert_eq!(attribute.get_all(), &vec![9u8, 9, 9, 9, 9, 9]);
    /// ```
    pub fn run(&self, map: &mut Map2d) {
        info!(
            "Create attribute '{}' of map '{}'",
            self.name,
            map.get_name()
        );

        map.create_attribute(self.name.clone(), self.default);
    }
}
