use std::time::Instant;

use crate::{engine::Engine, input::Key::Escape, types::SystemFn};

const MAX_FRAME_TIME: f32 = 0.25;

pub struct Application {
    pub engine: Engine,
    dt: f32,
    update: Vec<SystemFn>,
    render: Vec<SystemFn>,
    last: Instant,
    acc: f32,
}

impl Application {
    pub fn new(dt: f32) -> Self {
        Self {
            engine: Engine::new(dt),
            dt,
            update: Vec::new(),
            render: Vec::new(),
            last: Instant::now(),
            acc: 0.0,
        }
    }

    fn tick(&mut self) {
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

    fn render_frame(&mut self) {
        let Engine {
            world,
            commands,
            resources,
        } = &mut self.engine;
        for system in &self.render {
            system(world, resources, commands);
        }

        commands.run_on(world);
    }

    pub fn step(&mut self) {
        let now = Instant::now();
        self.acc += (now - self.last).as_secs_f32().min(MAX_FRAME_TIME);
        self.last = now;

        while self.acc >= self.dt {
            self.acc -= self.dt;
            self.tick();
        }

        self.render_frame();
    }

    pub fn should_quit(&self) {
        self.engine.resources.input.is_just_pressed(Escape);
    }

    pub fn add_update(mut self, system: SystemFn) -> Self {
        self.update.push(system);
        self
    }
    pub fn add_render(mut self, system: SystemFn) -> Self {
        self.render.push(system);
        self
    }
}
