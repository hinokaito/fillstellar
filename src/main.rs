mod renderer;
mod shape;
mod engine;

use std::sync::Arc;
use winit::{
    application::ApplicationHandler, 
    event::WindowEvent, 
    event_loop::{self, ActiveEventLoop, EventLoop}, 
    window::{Window, WindowId}
};

use engine::Engine;
use shape::ShapeType;

struct App {
    engine: Option<Engine>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.engine.is_none() {
            let window_attributes = Window::default_attributes().with_title("Fillstellar");
            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

            let mut engine = pollster::block_on(Engine::new(window));

            engine.spawn(ShapeType::Triangle);
            engine.spawn(ShapeType::Triangle);

            self.engine = Some(engine);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let engine = match self.engine.as_mut() {
            Some(e) => e,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => engine.resize(size),
            WindowEvent::RedrawRequested => engine.render(),
            _ => (),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    let mut app = App { engine: None };
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    event_loop.run_app(&mut app)?;
    Ok(())
}
