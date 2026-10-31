// ---------- константы ----------

use crate::voxels::chunk::{
    CHUNK_SIZE, CHUNK_SIZE_MASK, CHUNK_SIZE_SHIFT, CHUNK_VOLUME, Chunk, ChunkPos,
};

#[test]
fn constants_are_consistent() {
    assert_eq!(CHUNK_VOLUME, CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE);
    assert_eq!(CHUNK_SIZE, 1usize << CHUNK_SIZE_SHIFT);
    assert_eq!(CHUNK_SIZE_MASK, CHUNK_SIZE - 1);
    assert!(CHUNK_SIZE.is_power_of_two());
}

// ---------- new() ----------

#[test]
fn new_creates_empty_clean_chunk() {
    let c = Chunk::new(ChunkPos::new(2, -5, 7));
    assert_eq!(c.pos, ChunkPos::new(2, -5, 7));
    assert!(!c.dirty);
    assert_eq!(c.blocks.len(), CHUNK_VOLUME);
    assert!(
        c.blocks.iter().all(|&b| b == 0),
        "все блоки должны быть 0 (воздух)"
    );
}

#[test]
fn new_accepts_extreme_positions() {
    let c = Chunk::new(ChunkPos::new(i32::MIN, 0, i32::MAX));
    assert_eq!((c.pos.cx, c.pos.cy, c.pos.cz), (i32::MIN, 0, i32::MAX));
}

// ---------- index(): раскладка и биекция ----------

#[test]
fn index_layout_x_fastest_z_middle_y_slowest() {
    // Формула: x + y * SIZE^2 + z * SIZE
    // => X меняется быстрее всех (шаг 1), Z — средний (шаг SIZE),
    //    Y — самый медленный (шаг SIZE^2). Тест фиксирует этот порядок.
    assert_eq!(Chunk::index(0, 0, 0), 0);
    assert_eq!(Chunk::index(1, 0, 0), 1);
    assert_eq!(Chunk::index(CHUNK_SIZE - 1, 0, 0), CHUNK_SIZE - 1);
    assert_eq!(Chunk::index(0, 0, 1), CHUNK_SIZE);
    assert_eq!(Chunk::index(0, 0, 2), 2 * CHUNK_SIZE);
    assert_eq!(Chunk::index(0, 1, 0), CHUNK_SIZE * CHUNK_SIZE);
    // независимая проверка формулы на произвольной точке
    assert_eq!(
        Chunk::index(5, 2, 3),
        5 + 2 * CHUNK_SIZE * CHUNK_SIZE + 3 * CHUNK_SIZE
    );
    assert_eq!(
        Chunk::index(CHUNK_SIZE - 1, CHUNK_SIZE - 1, CHUNK_SIZE - 1),
        CHUNK_VOLUME - 1
    );
}

#[test]
fn index_is_bijective_on_chunk_volume() {
    // Каждый (x, y, z) -> уникальный индекс < CHUNK_VOLUME,
    // и все индексы 0..CHUNK_VOLUME покрыты (нет "дыр" и коллизий).
    let mut seen = vec![false; CHUNK_VOLUME];
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let i = Chunk::index(x, y, z);
                assert!(i < CHUNK_VOLUME, "index({x},{y},{z}) = {i} >= CHUNK_VOLUME");
                assert!(!seen[i], "индекс {i} дублируется для ({x},{y},{z})");
                seen[i] = true;
            }
        }
    }
    assert!(
        seen.iter().all(|&s| s),
        "не все индексы 0..CHUNK_VOLUME покрыты"
    );
}

// ---------- get / set ----------

#[test]
fn set_then_get_roundtrip() {
    let mut c = Chunk::new(ChunkPos::new(0, 0, 0));
    c.set(3, 4, 5, 777);
    assert_eq!(c.get(3, 4, 5), 777);
}

#[test]
fn set_get_u16_extremes() {
    let mut c = Chunk::new(ChunkPos::new(0, 0, 0));
    c.set(0, 0, 0, 0);
    assert_eq!(c.get(0, 0, 0), 0);
    c.set(CHUNK_SIZE - 1, CHUNK_SIZE - 1, CHUNK_SIZE - 1, u16::MAX);
    assert_eq!(
        c.get(CHUNK_SIZE - 1, CHUNK_SIZE - 1, CHUNK_SIZE - 1),
        u16::MAX
    );
}

#[test]
fn set_touches_only_target_cell() {
    let mut c = Chunk::new(ChunkPos::new(0, 0, 0));
    c.set(7, 8, 9, 12_345);
    for y in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let expected = if (x, y, z) == (7, 8, 9) { 12_345 } else { 0 };
                assert_eq!(c.get(x, y, z), expected, "в ({x},{y},{z})");
            }
        }
    }
}

#[test]
fn full_volume_write_readback() {
    // Пишем уникальные значения во все ячейки и читаем обратно:
    // любые коллизии индексов сразу проявятся.
    let mut c = Chunk::new(ChunkPos::new(0, 0, 0));
    let mut v: u16 = 0;
    for y in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                c.set(x, y, z, v);
                v = v.wrapping_add(1);
            }
        }
    }
    let mut v: u16 = 0;
    for y in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                assert_eq!(c.get(x, y, z), v, "в ({x},{y},{z})");
                v = v.wrapping_add(1);
            }
        }
    }
}

#[test]
fn blocks_slice_agrees_with_get_set() {
    let mut c = Chunk::new(ChunkPos::new(0, 0, 0));
    c.set(1, 2, 3, 42);
    assert_eq!(c.blocks[Chunk::index(1, 2, 3)], 42);
    assert_eq!(c.get(1, 2, 3), 42);
}

// ---------- dirty-флаг ----------

#[test]
fn get_does_not_mark_dirty() {
    let c = Chunk::new(ChunkPos::new(0, 0, 0));
    // читаем все 4096 ячеек
    for y in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let _ = c.get(x, y, z);
            }
        }
    }
    assert!(!c.dirty, "чтение не должно выставлять dirty");
}

#[test]
fn set_marks_dirty() {
    let mut c = Chunk::new(ChunkPos::new(0, 0, 0));
    c.set(0, 0, 0, 1);
    assert!(c.dirty);
}

#[test]
fn set_same_value_still_marks_dirty() {
    // Документирует текущее поведение: set() не сравнивает значения.
    let mut c = Chunk::new(ChunkPos::new(0, 0, 0));
    c.set(1, 1, 1, 5);
    c.clear_dirty();
    c.set(1, 1, 1, 5); // то же значение
    assert!(c.dirty);
}

#[test]
fn clear_dirty_resets_flag() {
    let mut c = Chunk::new(ChunkPos::new(0, 0, 0));
    c.set(0, 0, 0, 1);
    assert!(c.dirty);
    c.clear_dirty();
    assert!(!c.dirty);
    assert!(!c.consume_dirty(), "после clear_dirty consume вернёт false");
}

#[test]
fn consume_dirty_returns_previous_and_resets() {
    let mut c = Chunk::new(ChunkPos::new(0, 0, 0));
    assert!(!c.consume_dirty(), "чистый чанк -> false");
    assert!(!c.consume_dirty(), "и повторно -> false");

    c.set(0, 0, 0, 1);
    assert!(c.consume_dirty(), "после set -> true");
    assert!(!c.consume_dirty(), "пометка забрана -> false");
    assert!(!c.dirty);
}

#[test]
fn dirty_lifecycle_full_scenario() {
    // Типовой цикл: изменили -> перезаписали меш -> сбросили -> снова изменили.
    let mut c = Chunk::new(ChunkPos::new(1, 2, 3));
    assert!(!c.dirty);

    c.set(5, 5, 5, 100);
    assert!(c.dirty);
    c.clear_dirty(); // меш перезаписан
    assert!(!c.dirty);

    c.set(5, 5, 5, 200);
    assert!(c.dirty);
    assert!(c.consume_dirty()); // система ремешинга забрала пометку
    assert!(!c.dirty);

    c.set(6, 5, 5, 300);
    assert!(c.consume_dirty(), "новое изменение снова даёт true");
}

// ---------- паники на выходе за границы (только debug-сборка) ----------

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "assertion failed: x < CHUNK_SIZE")]
fn get_panics_in_debug_when_x_out_of_range() {
    let c = Chunk::new(ChunkPos::new(0, 0, 0));
    let _ = c.get(CHUNK_SIZE, 0, 0);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "assertion failed: x < CHUNK_SIZE")]
fn set_panics_in_debug_when_z_out_of_range() {
    let mut c = Chunk::new(ChunkPos::new(0, 0, 0));
    c.set(0, 0, CHUNK_SIZE, 1);
}

// ---------- Debug ----------

#[test]
fn debug_shows_pos_and_dirty_but_not_blocks() {
    let mut c = Chunk::new(ChunkPos::new(1, -2, 3));
    let s = format!("{c:?}");
    assert!(s.contains("Chunk"), "got: {s}");
    assert!(s.contains("cx: 1"), "got: {s}");
    assert!(s.contains("dirty: false"), "got: {s}");
    assert!(
        !s.contains("blocks"),
        "массив blocks не должен печататься: {s}"
    );

    c.set(0, 0, 0, 1);
    assert!(format!("{c:?}").contains("dirty: true"));
}

// ---------- интеграция с ChunkPos ----------

#[test]
fn world_coord_write_read() {
    // Типовой сценарий: мировые координаты -> чанк + локальные координаты.
    let (bx, by, bz) = (505, -20, 9);
    let mut chunk = Chunk::new(ChunkPos::from_block_pos(bx, by, bz));
    let (lx, ly, lz) = ChunkPos::local_index_of(bx, by, bz);
    assert!(lx < CHUNK_SIZE && ly < CHUNK_SIZE && lz < CHUNK_SIZE);

    chunk.set(lx, ly, lz, 42);
    assert_eq!(chunk.get(lx, ly, lz), 42);
    assert!(chunk.dirty);
    assert_eq!(chunk.pos, ChunkPos::new(31, -2, 0));
}

#[test]
fn two_chunks_are_independent() {
    let mut a = Chunk::new(ChunkPos::new(0, 0, 0));
    let mut b = Chunk::new(ChunkPos::new(1, 0, 0));
    a.set(1, 1, 1, 111);
    b.set(1, 1, 1, 222);
    assert_eq!(a.get(1, 1, 1), 111);
    assert_eq!(b.get(1, 1, 1), 222);
    assert!(a.dirty && b.dirty);
}
