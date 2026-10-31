use std::collections::HashMap;

use crate::voxels::{
    block::BlockPos,
    chunk::{Chunk, ChunkPos},
};

pub struct VoxelWorld {
    chunks: HashMap<ChunkPos, Chunk>,
}

impl VoxelWorld {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    #[inline]
    pub fn chunk(&self, pos: ChunkPos) -> Option<&Chunk> {
        self.chunks.get(&pos)
    }

    #[inline]
    pub fn chunk_mut(&mut self, pos: ChunkPos) -> Option<&mut Chunk> {
        self.chunks.get_mut(&pos)
    }

    /// Возвращает чанк, при необходимости создавая его.
    #[inline]
    pub fn ensure_chunk(&mut self, pos: ChunkPos) -> &mut Chunk {
        self.chunks.entry(pos).or_insert_with(|| Chunk::new(pos))
    }

    /// Позиции всех загруженных чанков (для генератора и выгрузки).
    pub fn loaded_chunks(&self) -> impl Iterator<Item = ChunkPos> + '_ {
        self.chunks.keys().copied()
    }

    pub fn dirty_chunks(&self) -> impl Iterator<Item = &ChunkPos> {
        self.chunks
            .iter()
            .filter(|(_, c)| c.dirty)
            .map(|(pos, _)| pos)
    }

    pub fn len(&self) -> usize {
        self.chunks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chunks.is_empty()
    }

    #[inline]
    pub fn get_block(&self, bx: i32, by: i32, bz: i32) -> Option<u16> {
        let pos = ChunkPos::from_block_pos(bx, by, bz);
        let chunk = self.chunks.get(&pos)?;
        let (lx, ly, lz) = ChunkPos::local_index_of(bx, by, bz);
        Some(chunk.get(lx, ly, lz))
    }

    /// Изменяет блок по мировой координате. Отсутствующий чанк создаётся.
    #[inline]
    pub fn set_block(&mut self, bx: i32, by: i32, bz: i32, block: u16) {
        let pos = ChunkPos::from_block_pos(bx, by, bz);
        let (lx, ly, lz) = ChunkPos::local_index_of(bx, by, bz);
        self.ensure_chunk(pos).set(lx, ly, lz, block);
    }

    pub fn set_block_batch(&mut self, blocks: &[(i32, i32, i32, u16)]) {
        for &(bx, by, bz, block) in blocks {
            self.set_block(bx, by, bz, block);
        }
    }

    #[inline]
    pub fn get_block_pos(&self, p: BlockPos) -> Option<u16> {
        self.get_block(p.bx, p.by, p.bz)
    }

    #[inline]
    pub fn set_block_pos(&mut self, p: BlockPos, block: u16) {
        self.set_block(p.bx, p.by, p.bz, block);
    }
}

impl Default for VoxelWorld {
    fn default() -> Self {
        Self::new()
    }
}
