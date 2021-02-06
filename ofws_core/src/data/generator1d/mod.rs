/// A trait to generate values for a 1d input.
pub trait Generator1d {
    /// Generates an output for an input.
    fn generate(&self, input: u32) -> u8;
}

/// Maps the input to the output.
pub struct InputToOutput;

impl Generator1d for InputToOutput {
    /// Generates an output for an input.
    fn generate(&self, input: u32) -> u8 {
        input as u8
    }
}
