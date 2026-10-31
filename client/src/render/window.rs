use std::sync::Arc;

pub struct Window {
    inner: Arc<winit::window::Window>,
}

impl Window {
    pub fn new(inner: winit::window::Window) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }

    pub fn inner(&self) -> &winit::window::Window {
        &self.inner
    }
}