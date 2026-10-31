use std::ops::{Add, Sub};

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_SIZE_SHIFT: u32 = 4;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub struct Chunk {
    pub blocks: Box<[u16; CHUNK_VOLUME]>, // 16x16x16
    pub pos: ChunkPos,
    pub dirty: bool,
}

impl Chunk {}

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

    /// Минимальная (мировая) X координата блока в этом чанке
    #[inline]
    pub const fn min_block_x(&self) -> i32 {
        self.cx << 4
    }

    /// Минимальная (мировая) Y координата блока в этом чанке
    #[inline]
    pub const fn min_block_y(&self) -> i32 {
        self.cy << 4
    }

    /// Минимальная (мировая) Z координата блока в этом чанке
    #[inline]
    pub const fn min_block_z(&self) -> i32 {
        self.cz << 4
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
