use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use engine::Application;

use crate::input_mapper::InputMapper;

/// Platform sloy: окно, события ОС, кадры.
pub struct AppHandler {
    app: Application,
    mapper: InputMapper,
    window: Option<Window>,
}

impl AppHandler {
    pub fn new(app: Application) -> Self {
        Self {
            app,
            mapper: InputMapper::new(),
            window: None,
        }
    }
}

impl ApplicationHandler for AppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = Window::default_attributes().with_title("Qubecat Engine");
        self.window = Some(
            event_loop
                .create_window(attrs)
                .expect("can't create window"),
        );
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => self.app.engine.resources.input.request_quit(),
            ref ev => self
                .mapper
                .handle_event(ev, &mut self.app.engine.resources.input),
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self
            .app
            .engine
            .resources
            .input
            .is_just_pressed(engine::input::Key::Escape)
        {
            self.app.engine.resources.input.request_quit();
        }

        if self.app.should_quit() {
            event_loop.exit();
            return;
        }

        self.app.step();
        if let Some(w) = &self.window {
            w.request_redraw();
        }

        // Кадр завершён: сбрасываем just_* до следующей пачки событий.
        self.app.engine.resources.input.begin_frame();
    }
}
