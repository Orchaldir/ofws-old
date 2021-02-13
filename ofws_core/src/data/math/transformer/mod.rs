use std::hash::Hash;

pub mod clusterer2d;
pub mod threshold;
pub mod transformer1d;
pub mod transformer2d;

pub trait Transformed: Default + Ord + Hash + Clone + Copy {}

impl Transformed for u8 {}
