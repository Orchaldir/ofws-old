use super::rendering::Renderer;

/// A trait to handle simple applications like the examples.
pub trait App {
    /// Initializes the application.
    fn init(&mut self) {}

    /// Renders the application.
    fn render(&mut self, renderer: &mut dyn Renderer);
}
