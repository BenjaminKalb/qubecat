/// Ввод: платформенный слой заполняет поля, системы только читают.
#[derive(Default, Clone, Copy)]
pub struct Input {
    pub quit: bool,    // Запрос на завершение работы
    pub fire: bool,    // Выстрел на этом тике
    pub move_dir: f32, // Горизонтальное движение, -1..1
}
