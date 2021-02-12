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
    /// Interpolates multiple elements.
    ///
    /// ```
    ///# use ofws_core::data::selector::Selector;
    /// let interpolator = Selector::InterpolateVector(vec![(100,150), (150,200), (200, 100)]);
    ///
    /// assert_eq!(interpolator.get(  0), 150);
    /// assert_eq!(interpolator.get( 50), 150);
    /// assert_eq!(interpolator.get(100), 150);
    /// assert_eq!(interpolator.get(125), 175);
    /// assert_eq!(interpolator.get(150), 200);
    /// assert_eq!(interpolator.get(175), 150);
    /// assert_eq!(interpolator.get(200), 100);
    /// assert_eq!(interpolator.get(255), 100);
    /// ```
    InterpolateVector(Vec<(u8, T)>),
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
            Selector::InterpolateVector(vector) => interpolate(vector, input),
            Selector::InterpolatePair { first, second } => {
                first.lerp(&second, input as f32 / 255.0)
            }
            Selector::Lookup(hashmap) => hashmap.get(&input).copied().unwrap_or_else(T::default),
        }
    }
}

fn interpolate<T: Selection>(vector: &[(u8, T)], input: u8) -> T {
    let mut last_entry = vector.get(0).unwrap();

    if input <= last_entry.0 {
        return last_entry.1;
    }

    for entry in &vector[1..] {
        if input <= entry.0 {
            let factor_in_interval =
                (input - last_entry.0) as f32 / (entry.0 - last_entry.0) as f32;
            return last_entry.1.lerp(&entry.1, factor_in_interval);
        }

        last_entry = entry;
    }

    last_entry.1
}
