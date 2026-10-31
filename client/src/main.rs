mod app_handler;
mod input_mapper;
mod render;

use engine::{Application, constants::ENGINE_DT};
use winit::event_loop::EventLoop;

use crate::app_handler::AppHandler;

fn main() {
    let app = Application::new(ENGINE_DT);
    let mut handler = AppHandler::new(app);

    let event_loop = EventLoop::builder().build().expect("cannot run event loop");

    event_loop
        .run_app(&mut handler)
        .expect("event loop завершился с ошибкой");
}
