use crate::data::math::interpolation::Interpolate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait Selection: Interpolate + Clone + Copy {}

impl Selection for u8 {}

#[derive(Debug, Serialize, Deserialize)]
pub struct InterpolateEntry<T: Selection> {
    threshold: u8,
    value: T,
}

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
    /// assert_eq!(interpolator.get(  0), 150);
    /// assert_eq!(interpolator.get( 50), 150);
    /// assert_eq!(interpolator.get(100), 150);
    /// assert_eq!(interpolator.get(125), 175);
    /// assert_eq!(interpolator.get(150), 200);
    /// assert_eq!(interpolator.get(175), 150);
    /// assert_eq!(interpolator.get(200), 100);
    /// assert_eq!(interpolator.get(255), 100);
    /// ```
    InterpolateVector(Vec<InterpolateEntry<T>>),
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

    /// Returns a VectorInterpolator, if the input is valid. It needs 2 or more elements:
    ///
    /// ```
    ///# use ofws_core::data::math::selector::Selector;
    /// assert!(Selector::new_interpolate_vector(vec![(0,50)]).is_err());
    /// ```
    ///
    /// The elements must be ordered based in their position:
    ///
    /// ```
    ///# use ofws_core::data::math::selector::Selector;
    /// assert!(Selector::new_interpolate_vector(vec![(50,50),(0,200)]).is_err());
    /// ```
    pub fn new_interpolate_vector(vector: Vec<(u8, T)>) -> Result<Selector<T>, &'static str> {
        if vector.len() < 2 {
            return Err("The vector needs at least 2 elements!");
        }

        let mut last_value = 0;

        for (value, _) in &vector {
            if *value < last_value {
                return Err("The elements of vector are not ordered!");
            }
            last_value = *value;
        }

        Ok(Selector::InterpolateVector(
            vector
                .iter()
                .map(|e| InterpolateEntry {
                    threshold: e.0,
                    value: e.1,
                })
                .collect::<Vec<_>>(),
        ))
    }

    pub fn new_lookup(lookup: HashMap<u8, T>, default: T) -> Selector<T> {
        Selector::Lookup { lookup, default }
    }

    /// Selects an object of type T based on the input.
    pub fn get(&self, input: u8) -> T {
        match self {
            Selector::Const(value) => *value,
            Selector::InterpolateVector(vector) => interpolate(vector, input),
            Selector::InterpolatePair { first, second } => {
                first.lerp(&second, input as f32 / 255.0)
            }
            Selector::Lookup { lookup, default } => lookup.get(&input).copied().unwrap_or(*default),
        }
    }
}

fn interpolate<T: Selection>(vector: &[InterpolateEntry<T>], input: u8) -> T {
    let mut last_entry = vector.get(0).unwrap();

    if input <= last_entry.threshold {
        return last_entry.value;
    }

    for entry in &vector[1..] {
        if input <= entry.threshold {
            let factor_in_interval = (input - last_entry.threshold) as f32
                / (entry.threshold - last_entry.threshold) as f32;
            return last_entry.value.lerp(&entry.value, factor_in_interval);
        }

        last_entry = entry;
    }

    last_entry.value
}
