mod game;

use engine::application::Application;

pub fn main() {
    let mut app = Application::new(1.0 / 30.0) // 30 тиков в секунду
        .add_update(game::demo_input)
        .add_update(game::player_control)
        .add_update(game::movement)
        .add_update(game::enemy_ai)
        .add_update(game::player_fire)
        .add_update(game::combat)
        .add_render(game::ascii_render);

    game::populate(&mut app.engine.world); // игрок и первая волна врагов
    app.run();
}