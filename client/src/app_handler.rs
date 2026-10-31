use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use engine::Application;

use crate::input_mapper::InputMapper;
use crate::render::context::GlContext;
use crate::render::shader::ShaderProgram;
use crate::render::triangle::Triangle;

pub struct AppHandler {
    app: Application,
    mapper: InputMapper,
    ctx: Option<GlContext>,
    program: Option<ShaderProgram>,
    triangle: Option<Triangle>,
}

impl AppHandler {
    pub fn new(app: Application) -> Self {
        Self {
            app,
            mapper: InputMapper::new(),
            ctx: None,
            program: None,
            triangle: None,
        }
    }
}

impl ApplicationHandler for AppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = Window::default_attributes().with_title("Qubecat Engine");
        let ctx = GlContext::create(event_loop, attrs).expect("не удалось создать GL-контекст");

        self.program = Some(ShaderProgram::new(&ctx.gl, VERTEX_SHADER, FRAGMENT_SHADER));
        self.triangle = Some(Triangle::new(&ctx.gl));
        self.ctx = Some(ctx);
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => self.app.engine.resources.input.request_quit(),
            WindowEvent::Resized(size) => {
                if let Some(ctx) = &mut self.ctx {
                    ctx.resize(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                let Some(ctx) = &mut self.ctx else {
                    return;
                };

                ctx.clear();
                if let (Some(program), Some(triangle)) = (&self.program, &self.triangle) {
                    program.bind(&ctx.gl);
                    triangle.draw(&ctx.gl);
                }
                ctx.present();
            }
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

        if let Some(ctx) = &self.ctx {
            ctx.window().request_redraw();
        }

        self.app.engine.resources.input.begin_frame();
    }
}

pub const VERTEX_SHADER: &str = r#"
#version 330 core
layout (location = 0) in vec3 position;

void main() {
    gl_Position = vec4(position, 1.0);
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
#version 330 core
out vec4 frag_color;

void main() {
    frag_color = vec4(0.2, 0.55, 0.9, 1.0);
}
"#;
