#[macro_use]
extern crate glium;
extern crate ofws_core;

mod builder;
pub mod renderer;
mod shader;
mod vertex;
pub mod window;

use crate::vertex::ColoredVertex;

implement_vertex!(ColoredVertex, position, color);
