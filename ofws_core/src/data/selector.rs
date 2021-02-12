use crate::data::math::interpolation::Interpolate;
use std::collections::HashMap;

pub trait Selection: Default + Interpolate + Clone + Copy {}

impl Selection for u8 {}

/// Selects an object of type T based on the input.
pub enum Selector<T: Selection> {
    /// Interpolates 2 elements.
    ///
    /// ```
    ///# use ofws_core::data::selector::Selector;
    /// let selector = Selector::new_interpolate_pair(100, 200);
    ///
    /// assert_eq!(selector.get(128), 150);
    /// ```
    InterpolatePair { first: T, second: T },
    /// Looks the input up in a hashmap or returns the default value.
    ///
    /// ```
    ///# use ofws_core::data::selector::Selector;
    /// let hashmap = vec![(1u8, 25u8), (3, 100)].into_iter().collect();
    /// let selector = Selector::Lookup(hashmap);
    ///
    /// assert_eq!(selector.get(0), 0);
    /// assert_eq!(selector.get(1), 25);
    /// assert_eq!(selector.get(2), 0);
    /// assert_eq!(selector.get(3), 100);
    /// assert_eq!(selector.get(4), 0);
    /// ```
    Lookup(HashMap<u8, T>),
}

impl<T: Selection> Selector<T> {
    pub fn new_interpolate_pair(first: T, second: T) -> Selector<T> {
        Selector::InterpolatePair { first, second }
    }

    /// Selects an object of type T based on the input.
    pub fn get(&self, input: u8) -> T {
        match self {
            Selector::InterpolatePair { first, second } => {
                first.lerp(&second, input as f32 / 255.0)
            }
            Selector::Lookup(hashmap) => hashmap.get(&input).copied().unwrap_or_else(T::default),
        }
    }
}
