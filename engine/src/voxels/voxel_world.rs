use std::collections::HashMap;

use crate::voxels::chunk::{Chunk, ChunkPos};

pub struct VoxelWorld {
    chunks: HashMap<ChunkPos, Chunk>,
}

impl VoxelWorld {
    pub fn get_block(bx: i32, by: i32, bz: i32) -> i32 {
        todo!()
    }
    pub fn set_block(bx: i32, by: i32, bz: i32) -> i32 {
        todo!()
    }
    pub fn set_block_batch(&[(bx, by, bz, id)]) {
        todo!()
    }
}
