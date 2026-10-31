use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowId};

use engine::Application;

pub struct AppHandler {
    app: Application,
    window: Option<Window>
}

impl AppHandler {
    pub fn new(app: Application) -> Self {
        Self { app, window: None }
    }

    fn request_quit(&mut self) {
        self.app.engine.resources.input.quit = true;
    }
}

impl ApplicationHandler for AppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = Window::default_attributes().with_title("Qubecat Engine");
        self.window = Some(
            event_loop
                .create_window(attrs)
                .expect("cannot create window"),
        );
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => self.request_quit(),
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed
                    && event.logical_key == Key::Named(NamedKey::Escape)
                {
                    self.request_quit();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.app.should_quit() {
            event_loop.exit();
            return;
        }
        self.app.step();
        // когда появится GL, рендер поедет из RedrawRequested, а не отсюда
        if let Some(w) = &self.window {
            w.request_redraw();
        }
    }

}