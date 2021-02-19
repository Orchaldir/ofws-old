use crate::data::math::interpolation::vector::VectorInterpolation;
use crate::data::math::interpolation::Interpolate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait Selection: Interpolate + Copy {}

impl Selection for u8 {}

/// Selects an object of type T based on the input.
#[derive(Debug, Serialize, Deserialize)]
pub enum Selector<T: Selection> {
    /// Returns a specific element.
    ///
    /// ```
    ///# use ofws_core::data::math::selector::Selector;
    /// assert_eq!(Selector::Const(99).get(128), 99);
    /// ```
    Const(T),
    /// Interpolates 2 elements.
    ///
    /// ```
    ///# use ofws_core::data::math::selector::Selector;
    /// let selector = Selector::new_interpolate_pair(100, 200);
    ///
    /// assert_eq!(selector.get(128), 150);
    /// ```
    InterpolatePair { first: T, second: T },
    /// Interpolates multiple elements.
    ///
    /// ```
    ///# use ofws_core::data::math::selector::Selector;
    /// let interpolator = Selector::new_interpolate_vector(vec![(100,150), (150,200), (200, 100)]).unwrap();
    ///
    /// assert_eq!(interpolator.get(125), 175);
    /// ```
    InterpolateVector(VectorInterpolation<u8, T>),
    /// Looks the input up in a hashmap or returns the default value.
    ///
    /// ```
    ///# use ofws_core::data::math::selector::Selector;
    /// let lookup = vec![(1u8, 25u8), (3, 100)].into_iter().collect();
    /// let selector = Selector::new_lookup(lookup, 1);
    ///
    /// assert_eq!(selector.get(0), 1);
    /// assert_eq!(selector.get(1), 25);
    /// assert_eq!(selector.get(2), 1);
    /// assert_eq!(selector.get(3), 100);
    /// assert_eq!(selector.get(4), 1);
    /// ```
    Lookup { lookup: HashMap<u8, T>, default: T },
}

impl<T: Selection> Selector<T> {
    pub fn new_interpolate_pair(first: T, second: T) -> Selector<T> {
        Selector::InterpolatePair { first, second }
    }

    pub fn new_interpolate_vector(vector: Vec<(u8, T)>) -> Result<Selector<T>, &'static str> {
        let interpolation = VectorInterpolation::new(vector)?;

        Ok(Selector::InterpolateVector(interpolation))
    }

    pub fn new_lookup(lookup: HashMap<u8, T>, default: T) -> Selector<T> {
        Selector::Lookup { lookup, default }
    }

    /// Selects an object of type T based on the input.
    pub fn get(&self, input: u8) -> T {
        match self {
            Selector::Const(value) => *value,
            Selector::InterpolatePair { first, second } => {
                first.lerp(&second, input as f32 / 255.0)
            }
            Selector::InterpolateVector(interpolation) => interpolation.interpolate(input),
            Selector::Lookup { lookup, default } => lookup.get(&input).copied().unwrap_or(*default),
        }
    }
}
