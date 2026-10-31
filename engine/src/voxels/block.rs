#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPos {
    pub bx: i32,
    pub by: i32,
    pub bz: i32,
}

impl BlockPos {
    #[inline]
    pub fn new(bx: i32, by: i32, bz: i32) -> Self {
        Self { bx, by, bz }
    }

    /// Чанк, в котором находится этот блок.
    #[inline]
    pub fn chunk(&self) -> crate::voxels::chunk::ChunkPos {
        crate::voxels::chunk::ChunkPos::from_block_pos(self.bx, self.by, self.bz)
    }

    /// Локальные координаты блока внутри его чанка (0..CHUNK_SIZE).
    #[inline]
    pub fn local(&self) -> (usize, usize, usize) {
        crate::voxels::chunk::ChunkPos::local_index_of(self.bx, self.by, self.bz)
    }
}
