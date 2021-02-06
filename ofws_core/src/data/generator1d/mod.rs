/// A trait to generate values for a 1d input.
pub trait Generator1d {
    /// Generates an output for an input.
    fn generate(&self, input: u32) -> u8;
}
