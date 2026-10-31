struct Chunk {
    blocks: [u16; 4096], // 16x16x16
    pos: ChunkPos,
    dirty: bool
}

impl Chunk {}

struct ChunkPos { cx: f32, cy: f32, cz: f32 }

impl ChunkPos {}