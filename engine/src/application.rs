use crate::{engine::Engine, types::SystemFn};

/// Приложение: движок плюс упоядоченные списки систем.
pub struct Application {
    pub engine: Engine,
    dt: f32,
    update: Vec<SystemFn>,
    render: Vec<SystemFn>,
}

impl Application {
    pub fn new(dt: f32) -> Self {
        Self {
            engine: Engine::new(dt),
            dt,
            update: Vec::new(),
            render: Vec::new(),
        }
    }

    fn tick(&mut self) {
        // Разбираем движок на поля: заимствования расходятся честно
        let Engine {
            world,
            commands,
            resources,
        } = &mut self.engine;
        for system in &self.update {
            system(world, resources, commands);
        }
        commands.run_on(world);
        resources.time.tick += 1;
        resources.time.elapsed += resources.time.dt;
    }

    /// Главный цикл: накопитель времени и фиксированный шаг.
    #[deprecated = "New render system is coming soon"]
    pub fn run(mut self) {
        let target_fps = 60.0; // рендер может не совпадать с тик-рейтом
        let frame_budget = std::time::Duration::from_secs_f32(1.0 / target_fps);

        let mut last = std::time::Instant::now();
        let mut acc = 0.0f32;
        while !self.engine.resources.input.quit {
            let frame_start = std::time::Instant::now(); // ① начало кадра
            acc += last.elapsed().as_secs_f32().min(0.25);
            last = std::time::Instant::now();
            while acc >= self.dt {
                acc -= self.dt;
                self.tick();
            }
            let Engine {
                world,
                commands,
                resources,
            } = &mut self.engine;
            for system in &self.render {
                system(world, resources, commands);
            }
            let frame_work = frame_start.elapsed(); // ② сколько уже потратили
            if frame_work < frame_budget {
                std::thread::sleep(frame_budget - frame_work); // ③ доспать до бюджета
            }
        }
    }

    pub fn step(&mut self) {

    }
    /// Порядок добавления = порядок исполнения.
    pub fn add_update(mut self, system: SystemFn) -> Self {
        self.update.push(system);
        self
    }
    pub fn add_render(mut self, system: SystemFn) -> Self {
        self.render.push(system);
        self
    }
}
