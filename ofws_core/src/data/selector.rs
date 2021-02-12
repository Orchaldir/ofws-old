use std::collections::HashMap;

pub trait Selection: Default + Clone + Copy {}

impl Selection for u8 {}

/// Selects an object of type T based on the input.
pub enum Selector<T: Selection> {
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
    /// Selects an object of type T based on the input.
    pub fn get(&self, input: u8) -> T {
        match self {
            Selector::Lookup(hashmap) => hashmap.get(&input).copied().unwrap_or_else(T::default),
        }
    }
}
