use ofws_core::interface::rendering::Point;

#[derive(Copy, Clone)]
pub struct ColoredVertex {
    pub position: Point,
    pub color: [f32; 3],
}

#[derive(Copy, Clone)]
pub struct TexturedVertex {
    pub position: Point,
    pub color: [f32; 3],
    pub tc: (f32, f32),
}
