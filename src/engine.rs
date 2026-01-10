use std::sync::Arc;
use winit::window::Window;
use crate::renderer::Renderer;
use crate::shape::{GameObject, ShapeType};

pub struct Engine {
    renderer: Renderer,
    objects: Vec<GameObject>,
}

impl Engine {
    pub async fn new(window: Arc<Window>) -> Self {
        let renderer = Renderer::new(window).await;
        Self {
            renderer,
            objects: Vec::new(),
        }
    }

    pub fn spawn(&mut self, shape: ShapeType) {
        self.objects.push(GameObject { shape });
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize(size);
    }

    pub fn render(&mut self) {
        match self.renderer.render(&self.objects) {
            Ok(_) => {},
            Err(wgpu::SurfaceError::Lost) => eprintln!("Surface LOST"),
            Err(wgpu::SurfaceError::OutOfMemory) => eprintln!("OUT OF MEMORY"),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}