use std::hash::Hash;

pub mod threshold;
pub mod transformer1d;

pub trait Transformed: Default + Ord + Hash + Clone + Copy {}

impl Transformed for u8 {}
