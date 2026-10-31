use engine::{CommandBuffer, Vec3, World, input::Key, resources::Resources};

/// Плоский мир: 2 чанка по X, два слоя камня. Генерируется один раз.
pub fn world_gen(_world: &mut World, res: &mut Resources, _cmd: &mut CommandBuffer) {
    if res.time.tick != 0 {
        return;
    }

    for cx in 0..2 {
        for cz in 0..2 {
            for y in 0..2 {
                for z in 0..16 {
                    for x in 0..16 {
                        res.voxels.set_block(cx * 16 + x, y, cz * 16 + z, 1);
                    }
                }
            }
        }
    }
}

/// Полёт по WASD + Space/Shift, поворот Q/E. Мышь — в Ф3.
pub fn player_move(_world: &mut World, res: &mut Resources, _cmd: &mut CommandBuffer) {
    let input = &res.input;
    let cam = &mut res.camera;
    let speed = 8.0 * res.time.dt;

    let mut delta = Vec3::ZERO;

    if input.is_held(Key::W) {
        delta += cam.forward();
    }
    if input.is_held(Key::S) {
        delta -= cam.forward();
    }
    if input.is_held(Key::D) {
        delta += cam.right();
    }
    if input.is_held(Key::A) {
        delta -= cam.right();
    }

    if input.is_held(Key::Space) {
        delta += Vec3::Y;
    }

    let shift_held = input.is_held(Key::LeftShift);
    if shift_held {
        delta -= Vec3::Y;
    }

    if delta.length_squared() > 0.0 {
        cam.position += delta.normalize() * speed;
    }

    const TURN: f32 = 2.0;
    if input.is_held(Key::Q) {
        cam.rotate(-TURN * res.time.dt, 0.0, 1.0);
    }
    if input.is_held(Key::E) {
        cam.rotate(TURN * res.time.dt, 0.0, 1.0);
    }
}
