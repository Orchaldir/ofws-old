use crate::renderer::GliumRenderer;
use glium::{glutin, Display};
use ofws_core::data::size2d::Size2d;
use ofws_core::interface::app::App;
use ofws_core::interface::window::Window;
use std::cell::RefCell;
use std::rc::Rc;

pub struct GliumWindow {
    title: &'static str,
    size: Size2d,
    tiles: Size2d,
    tile_size: Size2d,
}

impl GliumWindow {
    pub fn new(title: &'static str, tiles: Size2d, tile_size: Size2d) -> GliumWindow {
        let size = tiles * tile_size;
        GliumWindow {
            title,
            size,
            tiles,
            tile_size,
        }
    }

    pub fn default_size(title: &'static str) -> GliumWindow {
        GliumWindow::new(title, Size2d::new(40, 30), Size2d::new(20, 20))
    }

    fn create_display(&self, event_loop: &glutin::event_loop::EventLoop<()>) -> Display {
        let size = glutin::dpi::LogicalSize::new(self.size.width(), self.size.height());
        let wb = glutin::window::WindowBuilder::new()
            .with_title(self.title)
            .with_resizable(false)
            .with_inner_size(size);
        let cb = glutin::ContextBuilder::new();
        glium::Display::new(wb, cb, event_loop).unwrap()
    }
}

impl Window for GliumWindow {
    fn run(&mut self, app: Rc<RefCell<dyn App>>) -> ! {
        let event_loop = glutin::event_loop::EventLoop::new();
        let display = self.create_display(&event_loop);
        let mut renderer = GliumRenderer::new(display, self.tiles);

        {
            let mut reference = app.borrow_mut();
            reference.init();
        }

        event_loop.run(move |event, _, control_flow| {
            let next_frame_time =
                std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

            match event {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    _ => return,
                },
                glutin::event::Event::RedrawRequested(_) => (),
                _ => return,
            }

            let mut reference = app.borrow_mut();
            reference.render(&mut renderer);
        });
    }
}
