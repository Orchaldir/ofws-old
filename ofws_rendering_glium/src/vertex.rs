use ofws_core::interface::rendering::Point;

#[derive(Copy, Clone)]
pub struct ColoredVertex {
    pub position: Point,
    pub color: [u8; 3],
}
