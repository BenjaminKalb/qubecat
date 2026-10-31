use std::ops::{Add, Sub};

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_SIZE_SHIFT: u32 = 4;
pub const CHUNK_SIZE_MASK: usize = CHUNK_SIZE - 1; // 0b1111 для 16
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

const _: () = assert!(
    CHUNK_SIZE.is_power_of_two(),
    "CHUNK_SIZE must be a power of two"
);

pub struct Chunk {
    pub blocks: Box<[u16; CHUNK_VOLUME]>, // 16x16x16
    pub pos: ChunkPos,
    pub dirty: bool,
}

impl Chunk {
    #[inline]
    pub fn new(pos: ChunkPos) -> Self {
        Self {
            blocks: Box::new([0; CHUNK_VOLUME]),
            pos,
            dirty: false,
        }
    }

    #[inline]
    pub fn index(x: usize, y: usize, z: usize) -> usize {
        debug_assert!(x < CHUNK_SIZE && y < CHUNK_SIZE && z < CHUNK_SIZE);
        x + y * CHUNK_SIZE * CHUNK_SIZE + z * CHUNK_SIZE
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize, z: usize) -> u16 {
        self.blocks[Self::index(x, y, z)]
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, z: usize, block: u16) {
        self.blocks[Self::index(x, y, z)] = block;
        self.dirty = true;
    }
}

impl std::fmt::Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Chunk")
            .field("pos", &self.pos)
            .field("dirty", &self.dirty)
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub cx: i32,
    pub cy: i32,
    pub cz: i32,
}

impl ChunkPos {
    #[inline]
    pub fn new(cx: i32, cy: i32, cz: i32) -> Self {
        Self { cx, cy, cz }
    }

    /// Метод для получения координат чанка из мировых координат блока
    #[inline]
    pub fn from_block_pos(bx: i32, by: i32, bz: i32) -> Self {
        Self {
            cx: bx >> CHUNK_SIZE_SHIFT, // Быстрое деление на 16 через битовый сдвиг
            cy: by >> CHUNK_SIZE_SHIFT,
            cz: bz >> CHUNK_SIZE_SHIFT,
        }
    }

    #[inline]
    pub fn local_index_of(bx: i32, by: i32, bz: i32) -> (usize, usize, usize) {
        (
            (bx as usize) & CHUNK_SIZE_MASK,
            (by as usize) & CHUNK_SIZE_MASK,
            (bz as usize) & CHUNK_SIZE_MASK,
        )
    }

    /// Минимальная (мировая) X координата блока в этом чанке
    #[inline]
    pub const fn min_block_x(&self) -> i32 {
        self.cx << CHUNK_SIZE_SHIFT
    }

    /// Минимальная (мировая) Y координата блока в этом чанке
    #[inline]
    pub const fn min_block_y(&self) -> i32 {
        self.cy << CHUNK_SIZE_SHIFT
    }

    /// Минимальная (мировая) Z координата блока в этом чанке
    #[inline]
    pub const fn min_block_z(&self) -> i32 {
        self.cz << CHUNK_SIZE_SHIFT
    }

    /// Расстояние Чебышёва
    /// Формула: max(|x1 - x2|, |y1 - y2|, |z1 - z2|)
    #[inline]
    pub fn chess_distance(&self, other: &ChunkPos) -> i32 {
        let dx = (self.cx - other.cx).abs();
        let dy = (self.cy - other.cy).abs();
        let dz = (self.cz - other.cz).abs();

        dx.max(dy).max(dz)
    }
}

impl Add for ChunkPos {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.cx + rhs.cx, self.cy + rhs.cy, self.cz + rhs.cz)
    }
}

impl Sub for ChunkPos {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.cx - rhs.cx, self.cy - rhs.cy, self.cz - rhs.cz)
    }
}
