#[macro_use]
extern crate glium;
#[macro_use]
extern crate log;
extern crate ofws_core;

mod builder;
pub mod initialization;
mod input;
pub mod renderer;
mod shader;
mod texture;
mod vertex;
pub mod window;

use crate::vertex::ColoredVertex;
use crate::vertex::TexturedVertex;

implement_vertex!(ColoredVertex, position, color);
implement_vertex!(TexturedVertex, position, color, tc);
