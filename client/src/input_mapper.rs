use std::collections::HashMap;

use engine::input::{Input, Key, MouseButton};
use winit::{
    event::{ElementState, MouseButton as WinitMouseButton, MouseScrollDelta, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

pub struct InputMapper {
    bindings: HashMap<KeyCode, Key>,
}

impl InputMapper {
    pub fn new() -> Self {
        let mut mapper = Self {
            bindings: HashMap::new(),
        };

        mapper.load_default_bindings();
        mapper
    }

    fn load_default_bindings(&mut self) {
        self.bindings.insert(KeyCode::KeyW, Key::W);
        self.bindings.insert(KeyCode::KeyA, Key::A);
        self.bindings.insert(KeyCode::KeyS, Key::S);
        self.bindings.insert(KeyCode::KeyD, Key::D);
        self.bindings.insert(KeyCode::KeyQ, Key::Q);
        self.bindings.insert(KeyCode::KeyE, Key::E);
        self.bindings.insert(KeyCode::Space, Key::Space);
        self.bindings.insert(KeyCode::Escape, Key::Escape);
        self.bindings.insert(KeyCode::ShiftLeft, Key::LeftShift);
    }

    pub fn handle_event(&self, event: &WindowEvent, input: &mut Input) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                let PhysicalKey::Code(code) = event.physical_key else {
                    return;
                };

                let Some(&key) = self.bindings.get(&code) else {
                    return;
                };

                match event.state {
                    ElementState::Pressed => input.press(key),
                    ElementState::Released => input.release(key),
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                input.set_mouse_pos(position.x, position.y);
            }

            WindowEvent::MouseInput { state, button, .. } => {
                let Some(button) = map_mouse_button(*button) else {
                    return;
                };

                match state {
                    ElementState::Pressed => input.mouse_press(button),
                    ElementState::Released => input.mouse_release(button),
                }
            }

            WindowEvent::MouseWheel { delta, .. } => {
                let (dx, dy) = match delta {
                    MouseScrollDelta::LineDelta(x, y) => (*x, *y),

                    MouseScrollDelta::PixelDelta(position) => (
                        position.x as f32 / 20.0, // TODO: Replace this hack with proper scroll normalization
                        position.y as f32 / 20.0,
                    ),
                };

                input.add_mouse_scroll(dx, dy);
            }

            WindowEvent::Focused(false) => {
                input.release_all();
            }

            _ => {}
        }
    }
}

fn map_mouse_button(button: WinitMouseButton) -> Option<MouseButton> {
    match button {
        WinitMouseButton::Left => Some(MouseButton::Left),
        WinitMouseButton::Right => Some(MouseButton::Right),
        WinitMouseButton::Middle => Some(MouseButton::Middle),
        WinitMouseButton::Back => Some(MouseButton::X1),
        WinitMouseButton::Forward => Some(MouseButton::X2),
        WinitMouseButton::Other(_) => None,
    }
}

impl Default for InputMapper {
    fn default() -> Self {
        Self::new()
    }
}
