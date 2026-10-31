use crate::voxels::chunk::ChunkPos;

use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// ---------- new() ----------

#[test]
fn new_stores_components_as_is() {
    let p = ChunkPos::new(1, -2, 3);
    assert_eq!(p.cx, 1);
    assert_eq!(p.cy, -2);
    assert_eq!(p.cz, 3);
}

#[test]
fn new_accepts_extreme_values() {
    let p = ChunkPos::new(i32::MIN, 0, i32::MAX);
    assert_eq!((p.cx, p.cy, p.cz), (i32::MIN, 0, i32::MAX));
}

// ---------- from_block_pos() ----------

#[test]
fn from_block_pos_zero_and_positive() {
    assert_eq!(ChunkPos::from_block_pos(0, 0, 0), ChunkPos::new(0, 0, 0));
    // весь диапазон 0..=15 внутри чанка 0 — граница диапазона
    assert_eq!(ChunkPos::from_block_pos(15, 15, 15), ChunkPos::new(0, 0, 0));
    assert_eq!(ChunkPos::from_block_pos(16, 16, 16), ChunkPos::new(1, 1, 1));
    assert_eq!(
        ChunkPos::from_block_pos(255, 255, 255),
        ChunkPos::new(15, 15, 15)
    );
}

#[test]
fn from_block_pos_negative_floors_down() {
    // Арифметический сдвиг вправо для i32 — это деление с округлением
    // ВНИЗ (floor), а не к нулю, как обычное деление:
    assert_eq!(
        ChunkPos::from_block_pos(-1, -1, -1),
        ChunkPos::new(-1, -1, -1)
    );
    assert_eq!(
        ChunkPos::from_block_pos(-16, -16, -16),
        ChunkPos::new(-1, -1, -1)
    );
    assert_eq!(
        ChunkPos::from_block_pos(-17, -17, -17),
        ChunkPos::new(-2, -2, -2)
    );
    assert_eq!(
        ChunkPos::from_block_pos(-32, -33, -48),
        ChunkPos::new(-2, -3, -3)
    );
}

#[test]
fn from_block_pos_extremes() {
    assert_eq!(
        ChunkPos::from_block_pos(i32::MAX, i32::MAX, i32::MAX),
        ChunkPos::new(134_217_727, 134_217_727, 134_217_727)
    );
    assert_eq!(
        ChunkPos::from_block_pos(i32::MIN, i32::MIN, i32::MIN),
        ChunkPos::new(-134_217_728, -134_217_728, -134_217_728)
    );
}

#[test]
fn from_block_pos_matches_floor_division() {
    let samples = [
        -4096, -1023, -256, -255, -33, -32, -17, -16, -15, -1, 0, 1, 15, 16, 17, 255, 256, 4095,
    ];
    for &b in &samples {
        assert_eq!(
            ChunkPos::from_block_pos(b, 0, 0).cx,
            b.div_euclid(16),
            "bx = {b}"
        );
        assert_eq!(
            ChunkPos::from_block_pos(0, b, 0).cy,
            b.div_euclid(16),
            "by = {b}"
        );
        assert_eq!(
            ChunkPos::from_block_pos(0, 0, b).cz,
            b.div_euclid(16),
            "bz = {b}"
        );
    }
}

// ---------- local_index_of() ----------

#[test]
fn local_index_of_positive() {
    assert_eq!(ChunkPos::local_index_of(0, 0, 0), (0, 0, 0));
    assert_eq!(ChunkPos::local_index_of(5, 10, 15), (5, 10, 15));
    assert_eq!(ChunkPos::local_index_of(16, 32, 48), (0, 0, 0));
    assert_eq!(ChunkPos::local_index_of(31, 33, 47), (15, 1, 15));
}

#[test]
fn local_index_of_negative_wraps_into_chunk() {
    // Благодаря маске отрицательные координаты корректно
    // "заворачиваются" в локальный индекс внутри чанка:
    assert_eq!(ChunkPos::local_index_of(-1, -1, -1), (15, 15, 15));
    assert_eq!(ChunkPos::local_index_of(-16, -16, -16), (0, 0, 0));
    assert_eq!(ChunkPos::local_index_of(-17, -17, -17), (15, 15, 15));
}

#[test]
fn local_index_of_extremes() {
    assert_eq!(ChunkPos::local_index_of(i32::MIN, i32::MAX, 0), (0, 15, 0));
}

#[test]
fn local_index_of_matches_rem_euclid() {
    for &b in &[-33i32, -17, -16, -1, 0, 1, 15, 16, 31, 1000] {
        let (lx, ly, lz) = ChunkPos::local_index_of(b, b, b);
        let expected = b.rem_euclid(16) as usize;
        assert_eq!((lx, ly, lz), (expected, expected, expected), "b = {b}");
    }
}

// ---------- min_block_*() ----------

#[test]
fn min_block_coords() {
    let c = ChunkPos::new(0, 1, -2);
    assert_eq!(c.min_block_x(), 0);
    assert_eq!(c.min_block_y(), 16);
    assert_eq!(c.min_block_z(), -32);
}

#[test]
fn min_block_coords_are_const_usable() {
    // min_block_* объявлены const fn — проверяем, что их можно
    // вызывать в константном контексте.
    const C: ChunkPos = ChunkPos {
        cx: 3,
        cy: 4,
        cz: 5,
    };
    const MIN: (i32, i32, i32) = (C.min_block_x(), C.min_block_y(), C.min_block_z());
    assert_eq!(MIN, (48, 64, 80));
}

// ---------- согласованность chunk/local ----------

#[test]
fn chunk_plus_local_reconstructs_block_pos() {
    for x in (-64..=64).step_by(3) {
        for y in (-64..=64).step_by(7) {
            for z in (-64..=64).step_by(5) {
                let chunk = ChunkPos::from_block_pos(x, y, z);
                let (lx, ly, lz) = ChunkPos::local_index_of(x, y, z);
                assert_eq!(chunk.min_block_x() + lx as i32, x, "x = {x}");
                assert_eq!(chunk.min_block_y() + ly as i32, y, "y = {y}");
                assert_eq!(chunk.min_block_z() + lz as i32, z, "z = {z}");
            }
        }
    }
}

#[test]
fn from_block_pos_of_min_block_returns_same_chunk() {
    for &c in &[-1000, -1, 0, 1, 999] {
        let chunk = ChunkPos::new(c, c, c);
        let again = ChunkPos::from_block_pos(
            chunk.min_block_x(),
            chunk.min_block_y(),
            chunk.min_block_z(),
        );
        assert_eq!(again, chunk);
    }
}

// ---------- chess_distance() ----------

#[test]
fn chess_distance_to_self_is_zero() {
    let p = ChunkPos::new(7, -3, 12);
    assert_eq!(p.chess_distance(&p), 0);
}

#[test]
fn chess_distance_unit_neighbors() {
    let base = ChunkPos::new(10, -10, 10);
    for &d in &[
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
        // диагональные соседи тоже на расстоянии 1 (это Чебышёв)
        (1, 1, 1),
        (-1, 1, -1),
    ] {
        let neighbor = base + ChunkPos::new(d.0, d.1, d.2);
        assert_eq!(base.chess_distance(&neighbor), 1, "d = {d:?}");
    }
}

#[test]
fn chess_distance_is_max_of_axis_deltas() {
    let base = ChunkPos::new(0, 0, 0);
    assert_eq!(base.chess_distance(&ChunkPos::new(3, 7, 2)), 7);
    assert_eq!(base.chess_distance(&ChunkPos::new(10, -2, 4)), 10);
    assert_eq!(base.chess_distance(&ChunkPos::new(-13, 5, 1)), 13);
}

#[test]
fn chess_distance_is_symmetric() {
    let a = ChunkPos::new(-12, 5, 40);
    let b = ChunkPos::new(8, -15, 0);
    assert_eq!(a.chess_distance(&b), b.chess_distance(&a));
    assert_eq!(a.chess_distance(&b), 40);
}

#[test]
fn chess_distance_matches_formula_on_grid() {
    for x in -3..=3 {
        for y in -2..=2 {
            for z in -3..=3 {
                let a = ChunkPos::new(x, y, z);
                let b = ChunkPos::new(3 - x, -y, x + z);
                let expected = (a.cx - b.cx)
                    .abs()
                    .max((a.cy - b.cy).abs())
                    .max((a.cz - b.cz).abs());
                assert_eq!(a.chess_distance(&b), expected, "a = {a:?}, b = {b:?}");
            }
        }
    }
}

// ---------- Add / Sub ----------

#[test]
fn add_is_componentwise() {
    assert_eq!(
        ChunkPos::new(1, 2, 3) + ChunkPos::new(10, 20, 30),
        ChunkPos::new(11, 22, 33)
    );
    assert_eq!(
        ChunkPos::new(-5, 0, 7) + ChunkPos::new(2, -3, -7),
        ChunkPos::new(-3, -3, 0)
    );
}

#[test]
fn add_zero_is_identity() {
    let a = ChunkPos::new(-9, 13, 42);
    assert_eq!(a + ChunkPos::new(0, 0, 0), a);
    assert_eq!(ChunkPos::new(0, 0, 0) + a, a);
}

#[test]
fn sub_is_componentwise() {
    assert_eq!(
        ChunkPos::new(10, 20, 30) - ChunkPos::new(1, 2, 3),
        ChunkPos::new(9, 18, 27)
    );
    assert_eq!(
        ChunkPos::new(1, 2, 3) - ChunkPos::new(4, 5, 6),
        ChunkPos::new(-3, -3, -3)
    );
}

#[test]
fn add_then_sub_restores_original() {
    let a = ChunkPos::new(-100, 55, 7);
    let b = ChunkPos::new(3, -2, 999);
    assert_eq!((a + b) - b, a);
    assert_eq!(a + (b - a), b);
}

#[test]
fn neighbor_offsets_typical_usage() {
    // Типовой сценарий: базовый чанк + смещение к соседу.
    let base = ChunkPos::from_block_pos(500, -33, 7); // => (31, -3, 0)
    assert_eq!(base, ChunkPos::new(31, -3, 0));

    let east = base + ChunkPos::new(1, 0, 0);
    assert_eq!(east, ChunkPos::from_block_pos(516, -33, 7));

    let down = base + ChunkPos::new(0, -1, 0);
    assert_eq!(down, ChunkPos::from_block_pos(500, -49, 7));
}

// ---------- выводные типажи (Debug, Copy, PartialEq, Hash) ----------

#[test]
fn equality_and_inequality() {
    assert_eq!(ChunkPos::new(1, 2, 3), ChunkPos::new(1, 2, 3));
    assert_ne!(ChunkPos::new(1, 2, 3), ChunkPos::new(3, 2, 1));
    assert_ne!(ChunkPos::new(1, 2, 3), ChunkPos::new(1, 2, 4));
}

#[test]
fn is_copy_and_still_usable_after_assignment() {
    let a = ChunkPos::new(4, 5, 6);
    let b = a; // Copy, а не move
    assert_eq!(a, b);
}

#[test]
fn equal_values_have_equal_hashes() {
    let (a, b) = (ChunkPos::new(-7, 0, 9), ChunkPos::new(-7, 0, 9));
    let mut h1 = DefaultHasher::new();
    let mut h2 = DefaultHasher::new();
    a.hash(&mut h1);
    b.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn works_as_hash_set_key() {
    let mut loaded = HashSet::new();
    loaded.insert(ChunkPos::new(0, 0, 0));
    loaded.insert(ChunkPos::new(0, 0, 0)); // дубликат не добавится
    loaded.insert(ChunkPos::new(1, 0, 0));
    assert_eq!(loaded.len(), 2);
    assert!(loaded.contains(&ChunkPos::new(0, 0, 0)));
}

#[test]
fn debug_prints_all_fields() {
    let s = format!("{:?}", ChunkPos::new(1, -2, 3));
    assert!(s.contains("cx: 1"), "got: {s}");
    assert!(s.contains("cy: -2"), "got: {s}");
    assert!(s.contains("cz: 3"), "got: {s}");
}
