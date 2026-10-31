use std::ffi::CString;
use std::num::NonZeroU32;

use glow::HasContext as _;
use glutin::context::{ContextAttributesBuilder, PossiblyCurrentContext, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{GlSurface, Surface, SurfaceAttributesBuilder, SwapInterval, WindowSurface};
use glutin_winit::GlWindow;
use winit::event_loop::ActiveEventLoop;
use winit::raw_window_handle::HasWindowHandle;
use winit::window::{Window, WindowAttributes};

pub struct GlContext {
    window: Window,
    surface: Surface<WindowSurface>,
    context: PossiblyCurrentContext,
    pub gl: glow::Context,
}

impl GlContext {
    /// Создаёт окно вместе с GL-контекстом, согласованным по пиксельному формату.
    pub fn create(
        event_loop: &ActiveEventLoop,
        attrs: WindowAttributes,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let (window, config) = glutin_winit::DisplayBuilder::new()
            .with_window_attributes(Some(attrs))
            .build(
                event_loop,
                glutin::config::ConfigTemplateBuilder::new(),
                |mut cfgs| cfgs.next().expect("нет подходящего GL-конфига"),
            )?;
        let window = window.expect("DisplayBuilder не создал окно");

        let ctx_attrs = ContextAttributesBuilder::new()
            .with_context_api(glutin::context::ContextApi::OpenGl(Some(Version::new(
                3, 3,
            ))))
            .build(Some(
                window
                    .window_handle()
                    .expect("окно не дало handle")
                    .as_raw(),
            ));

        let not_current = unsafe { config.display().create_context(&config, &ctx_attrs)? };

        let surface_attrs =
            window.build_surface_attributes(SurfaceAttributesBuilder::<WindowSurface>::new())?;
        let surface = unsafe {
            config
                .display()
                .create_window_surface(&config, &surface_attrs)?
        };

        let context = not_current.make_current(&surface)?;

        let gl = unsafe {
            let display = surface.display().clone();
            glow::Context::from_loader_function(move |name| {
                display.get_proc_address(&CString::new(name).expect("имя функции без NUL"))
            })
        };

        // Вертикальная синхронизация: 1 кадр на обновление монитора.
        let vsync = SwapInterval::Wait(NonZeroU32::new(1).expect("1 != 0"));
        let _ = surface.set_swap_interval(&context, vsync);

        Ok(Self {
            window,
            surface,
            context,
            gl,
        })
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    /// Пересоздаёт/растягивает поверхность под новый размер окна.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.window.resize_surface(&self.surface, &self.context);
    }

    /// Начало кадра: очистка буферов цвета/глубины.
    pub fn clear(&mut self) {
        unsafe {
            self.gl.clear_color(0.08, 0.09, 0.12, 1.0);
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    /// Конец кадра: показ нарисованного на экран.
    pub fn present(&mut self) {
        self.surface
            .swap_buffers(&self.context)
            .expect("swap_buffers не удался");
    }
}
