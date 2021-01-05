use super::rendering::{Initialization, Renderer};

/// A trait to handle simple applications like the examples.
pub trait App {
    /// Initializes the application.
    fn init(&mut self, _initialization: &mut dyn Initialization) {}

    /// Renders the application.
    fn render(&mut self, renderer: &mut dyn Renderer);
}
