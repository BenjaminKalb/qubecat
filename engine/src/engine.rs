use hecs::{CommandBuffer, World};

use crate::{input::Input, resources::Resources, time::Time};

/// Состояние движка: мир, ресурсы и буфер команд.
pub struct Engine {
    pub world: World,
    pub commands: CommandBuffer,
    pub resources: Resources,
}

impl Engine {
    pub fn new(dt: f32) -> Self {
        Self {
            world: World::new(),
            commands: CommandBuffer::new(),
            resources: Resources {
                time: Time {
                    tick: 0,
                    dt,
                    elapsed: 0.0,
                },
                input: Input::default(),
            },
        }
    }
}
