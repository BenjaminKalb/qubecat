use engine::{CommandBuffer, Entity, World, resources::Resources};


#[derive(Clone, Copy)]
pub struct Position { pub x: f32, pub y: f32 }
#[derive(Clone, Copy)]
pub struct Velocity { pub vx: f32, pub vy: f32 }

pub struct Player; // Маркер: управляемая сущность
pub struct Enemy; // Маркер: противник
pub struct Bullet; // Маркер: снаряд

pub const WIDTH: usize = 44; // Ширина экрана в символах
pub const HEIGHT: usize = 14; // Высота экрана в символах

/// Начальное наполнение мира.
pub fn populate(world: &mut World) {
    world.spawn((
        Position { x: 22.0, y: 2.0 },
        Velocity { vx: 0.0, vy: 0.0 },
        Player {},
    ));
    spawn_wave(world, 10.0, 6);
}

fn spawn_wave(world: &mut World, y: f32, count: usize) {
    for i in 0..count {
        world.spawn((
            Position { x: 4.0 + i as f32 * 6.0, y },
            Velocity { vx: 6.0, vy: 0.0 },
            Enemy,
        ));
    }
}

/// Детерминированный сценарий вместо клавиатуры.
pub fn demo_input(_w: &mut World, res: &mut Resources, _c: &mut CommandBuffer) {
    res.input.fire = res.time.tick % 10 == 0;
    res.input.move_dir = if (res.time.tick / 40) % 2 == 0 { 1.0 } else { -1.0 };
    res.input.quit = res.time.tick >= 200;
}

/// Ввод игрока превращается в его Velocity.
pub fn player_control(w: &mut World, res: &mut Resources, _c: &mut CommandBuffer) {
    let dir = res.input.move_dir;
    for (_e, vel) in w.query_mut::<(&mut Velocity, &Player)>() {
        vel.vx = dir * 5.0;
    }
}

/// Интеграция движения: один запрос - одна система.
pub fn movement(w: &mut World, res: &mut Resources, _c: &mut CommandBuffer) {
    let dt = res.time.dt;
    for (_e, pos, vel) in w.query_mut::<(Entity, &mut Position, &mut Velocity)>() {
        pos.x += vel.vx * dt;
        pos.y += vel.vy * dt;
    }
}

/// Враги отражаются от краев и спускаются к игроку.
pub fn enemy_ai(w: &mut World, res: &mut Resources, _c: &mut CommandBuffer) {
    for (_e, pos, vel) in w.query_mut::<(Entity, &mut Position, &mut Velocity, &Enemy)>() {
        if pos.x <= 1.0 || pos.x >= (WIDTH - 2) as f32 {
            vel.vx = -vel.vx;
            pos.y += 0.8;
        }
    }
}

/// Выстрел: спавн пули откладывается в CommandBuffer.
pub fn player_fire(w: &mut World, res: &mut Resources, cmd: &mut CommandBuffer) {
    if !res.input.fire {
        return;
    }
    // Позиция игрока: игрок в демо всегда один
    let Some((_, pos, _)) = w.query::<(Entity, &Position, &Player)>().iter().next()
    else {
        return;
    };
    let pos = *pos;
    cmd.spawn((
        Position { x: pos.x, y: pos.y + 1.0 },
        Velocity { vx: 0.0, vy: 12.0 },
        Bullet,
    ));
}

/// Коллизия "пуля=враг": снимки плюс отложенный деспавн.
pub fn combat(w: &mut World, _res: &mut Resources, cmd: &mut CommandBuffer) {
    // Снимок пуль: копируем данные, мир во время обхода не меняется
    let bullets: Vec<(Entity, Position)> = w
        .query::<(Entity, &Position, &Bullet)>()
        .iter()
        .map(|(e, p, _)| (e, *p))
        .collect();

    // Снимок врагов
    let enemies: Vec<(Entity, Position)> = w
        .query::<(Entity, &Position, &Enemy)>()
        .iter()
        .map(|(e, p, _)| (e, *p))
        .collect();

    let mut used: Vec<Entity> = Vec::new(); // пули, потраченные в этом тике
    for &(b, bpos) in &bullets {
        for &(e, epos) in &enemies {
            if !used.contains(&b)
                && (epos.x - bpos.x).abs() < 1.0
                && (epos.y - bpos.y).abs() < 1.0
            {
                cmd.despawn(b);
                cmd.despawn(e);
                used.push(b);
            }
        }
    }

    // Пули, улетающие за экран, убираем тоже - но отложено
    let gone: Vec<(Entity)> = w
        .query::<(Entity, &Position, &Bullet)>()
        .iter()
        .filter(|(_, p, _)| p.y > (HEIGHT - 1) as f32)
        .map(|(e, _, _)| e)
        .collect();
    for e in gone {
        cmd.despawn(e);
    }
}

/// ASCII-render: каждая группа сущностей - отдельный читающий запрос.
pub fn ascii_render(w: &mut World, res: &mut Resources, _c: &mut CommandBuffer) {
    let mut screen = ['.'; WIDTH * HEIGHT];

    for (_e, pos, _b) in w.query::<(Entity, &Position, &Bullet)>().iter() {
        put(&mut screen, pos, '|');
    }

    for (_e, pos, _n) in w.query::<(Entity, &Position, &Enemy)>().iter() {
        put(&mut screen, pos, 'W');
    }

    for (_e, pos, _p) in w.query::<(Entity, &Position, &Player)>().iter() {
        put(&mut screen, pos, 'A');
    }

    println!("tick {:3} | сущностей: {}", res.time.tick, w.len());
    for row in screen.chunks(WIDTH) {
        println!("{}", row.iter().collect::<String>());
    }
}

/// Запись глифа с проверкой границ.
fn put(screen: &mut [char], pos: &Position, glyph: char) {
    let (x, y) = (pos.x as i32, pos.y as i32);
    if (0..WIDTH as i32).contains(&x) && (0..HEIGHT as i32).contains(&y) {
        screen[y as usize * WIDTH + x as usize] = glyph;
    }
}