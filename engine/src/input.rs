use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    W,
    A,
    S,
    D,
    Space,
    Escape,
    LeftShift,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    X1,
    X2,
}

#[derive(Debug, Default)]
pub struct MouseState {
    position: (f64, f64),
    scroll_delta: (f32, f32),

    buttons_held: HashSet<MouseButton>,
    buttons_just_pressed: HashSet<MouseButton>,
    buttons_just_released: HashSet<MouseButton>,
}

#[derive(Debug, Default)]
pub struct Input {
    /// Запрос на завершение: ставится CloseRequested/Escape, читается AppHandler.
    quit_requested: bool,
    keys_held: HashSet<Key>,
    keys_just_pressed: HashSet<Key>,
    keys_just_released: HashSet<Key>,

    mouse: MouseState,
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    /// Запрос платформы/обработчика закрыть приложение.
    pub fn request_quit(&mut self) {
        self.quit_requested = true;
    }

    /// Прочитан AppHandler'ом: пора ли завершать.
    pub fn quit_requested(&self) -> bool {
        self.quit_requested
    }

    // Keyboard
    pub fn is_held(&self, key: Key) -> bool {
        self.keys_held.contains(&key)
    }

    pub fn is_just_pressed(&self, key: Key) -> bool {
        self.keys_just_pressed.contains(&key)
    }

    pub fn is_just_released(&self, key: Key) -> bool {
        self.keys_just_released.contains(&key)
    }

    pub fn held_keys(&self) -> impl Iterator<Item = &Key> {
        self.keys_held.iter()
    }

    pub fn press(&mut self, key: Key) {
        if self.keys_held.insert(key) {
            self.keys_just_pressed.insert(key);
        }
    }

    pub fn release(&mut self, key: Key) {
        if self.keys_held.remove(&key) {
            self.keys_just_released.insert(key);
        }
    }

    // Mouse
    pub fn mouse_pos(&self) -> (f64, f64) {
        self.mouse.position
    }

    pub fn mouse_scroll(&self) -> (f32, f32) {
        self.mouse.scroll_delta
    }

    pub fn is_mouse_held(&self, button: MouseButton) -> bool {
        self.mouse.buttons_held.contains(&button)
    }

    pub fn is_mouse_just_pressed(&self, button: MouseButton) -> bool {
        self.mouse.buttons_just_pressed.contains(&button)
    }

    pub fn is_mouse_just_released(&self, button: MouseButton) -> bool {
        self.mouse.buttons_just_released.contains(&button)
    }

    pub fn held_mouse_buttons(&self) -> impl Iterator<Item = &MouseButton> {
        self.mouse.buttons_held.iter()
    }

    pub fn set_mouse_pos(&mut self, x: f64, y: f64) {
        self.mouse.position = (x, y);
    }

    pub fn add_mouse_scroll(&mut self, dx: f32, dy: f32) {
        self.mouse.scroll_delta.0 += dx;
        self.mouse.scroll_delta.1 += dy;
    }

    pub fn mouse_press(&mut self, button: MouseButton) {
        if self.mouse.buttons_held.insert(button) {
            self.mouse.buttons_just_pressed.insert(button);
        }
    }

    pub fn mouse_release(&mut self, button: MouseButton) {
        if self.mouse.buttons_held.remove(&button) {
            self.mouse.buttons_just_released.insert(button);
        }
    }

    /// Вызывать один раз в начале кадра, до обработки событий.
    pub fn begin_frame(&mut self) {
        self.keys_just_pressed.clear();
        self.keys_just_released.clear();

        self.mouse.buttons_just_pressed.clear();
        self.mouse.buttons_just_released.clear();

        self.mouse.scroll_delta = (0.0, 0.0);
    }

    /// Сбрасывает удерживаемые клавиши и кнопки мыши.
    /// События отпускания доступны до следующего begin_frame().
    pub fn release_all(&mut self) {
        for key in self.keys_held.drain() {
            self.keys_just_released.insert(key);
        }

        for button in self.mouse.buttons_held.drain() {
            self.mouse.buttons_just_released.insert(button);
        }
    }
}
