use crate::voxels::{
    chunk::{CHUNK_SIZE, Chunk, ChunkPos},
    voxel_world::VoxelWorld,
};

const AIR: u16 = 0;

pub struct MeshData {
    pub positions: Vec<f32>,
    pub block_ids: Vec<f32>,
    pub indices: Vec<u32>,
}

struct Face {
    neighbor: [i32; 3],
    corners: [[f32; 3]; 4],
}

// Вершины каждой грани перечислены против часовой стрелки
// при взгляде с внешней стороны куба.
const FACES: [Face; 6] = [
    // +X
    Face {
        neighbor: [1, 0, 0],
        corners: [
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [1.0, 1.0, 1.0],
            [1.0, 0.0, 1.0],
        ],
    },
    // -X
    Face {
        neighbor: [-1, 0, 0],
        corners: [
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 1.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0],
        ],
    },
    // +Y
    Face {
        neighbor: [0, 1, 0],
        corners: [
            [0.0, 1.0, 1.0],
            [1.0, 1.0, 1.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ],
    },
    // -Y
    Face {
        neighbor: [0, -1, 0],
        corners: [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ],
    },
    // +Z
    Face {
        neighbor: [0, 0, 1],
        corners: [
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [1.0, 1.0, 1.0],
            [0.0, 1.0, 1.0],
        ],
    },
    // -Z
    Face {
        neighbor: [0, 0, -1],
        corners: [
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
        ],
    },
];

impl MeshData {
    pub fn mesh_chunk(world: &VoxelWorld, pos: ChunkPos) -> Option<MeshData> {
        let chunk = world.chunk(pos)?;

        // Нет смысла генерировать геометрию пустого чанка.
        if chunk.is_empty() {
            return None;
        }

        let origin_x = pos.min_block_x();
        let origin_y = pos.min_block_y();
        let origin_z = pos.min_block_z();

        let mut mesh = MeshData {
            positions: Vec::new(),
            block_ids: Vec::new(),
            indices: Vec::new(),
        };

        // X меняется быстрее всего согласно Chunk::index().
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let block_id = chunk.get(x, y, z);

                    if block_id == AIR {
                        continue;
                    }

                    let bx = origin_x + x as i32;
                    let by = origin_y + y as i32;
                    let bz = origin_z + z as i32;

                    for face in &FACES {
                        if !Self::is_face_visible(world, chunk, x, y, z, bx, by, bz, face) {
                            continue;
                        }

                        Self::emit_face(&mut mesh, face, block_id, bx, by, bz);
                    }
                }
            }
        }

        Some(mesh)
    }

    #[inline]
    fn is_face_visible(
        world: &VoxelWorld,
        chunk: &Chunk,
        x: usize,
        y: usize,
        z: usize,
        bx: i32,
        by: i32,
        bz: i32,
        face: &Face,
    ) -> bool {
        let nx = x as i32 + face.neighbor[0];
        let ny = y as i32 + face.neighbor[1];
        let nz = z as i32 + face.neighbor[2];

        // Сосед находится внутри текущего чанка.
        if nx >= 0
            && nx < CHUNK_SIZE as i32
            && ny >= 0
            && ny < CHUNK_SIZE as i32
            && nz >= 0
            && nz < CHUNK_SIZE as i32
        {
            return chunk.get(nx as usize, ny as usize, nz as usize) == AIR;
        }

        // Сосед находится в другом чанке.
        let Some(nx) = bx.checked_add(face.neighbor[0]) else {
            return true;
        };

        let Some(ny) = by.checked_add(face.neighbor[1]) else {
            return true;
        };

        let Some(nz) = bz.checked_add(face.neighbor[2]) else {
            return true;
        };

        // Незагруженный чанк считается воздухом.
        world.get_block(nx, ny, nz).map_or(true, |id| id == AIR)
    }

    #[inline]
    fn emit_face(mesh: &mut MeshData, face: &Face, block_id: u16, bx: i32, by: i32, bz: i32) {
        let base = (mesh.positions.len() / 3) as u32;

        for corner in &face.corners {
            mesh.positions.push(bx as f32 + corner[0]);
            mesh.positions.push(by as f32 + corner[1]);
            mesh.positions.push(bz as f32 + corner[2]);

            mesh.block_ids.push(block_id as f32);
        }

        mesh.indices
            .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_block_generates_six_faces() {
        let mut world = VoxelWorld::new();

        let pos = ChunkPos::new(0, 0, 0);
        world.set_block(0, 0, 0, 1);

        let mesh = MeshData::mesh_chunk(&world, pos).unwrap();

        assert_eq!(mesh.positions.len(), 24 * 3);
        assert_eq!(mesh.block_ids.len(), 24);
        assert_eq!(mesh.indices.len(), 36);
    }

    #[test]
    fn adjacent_blocks_generate_ten_faces() {
        let mut world = VoxelWorld::new();

        let pos = ChunkPos::new(0, 0, 0);

        world.set_block(0, 0, 0, 1);
        world.set_block(1, 0, 0, 1);

        let mesh = MeshData::mesh_chunk(&world, pos).unwrap();

        assert_eq!(mesh.positions.len(), 40 * 3);
        assert_eq!(mesh.block_ids.len(), 40);
        assert_eq!(mesh.indices.len(), 60);
    }

    #[test]
    fn empty_chunk_returns_none() {
        let mut world = VoxelWorld::new();

        let pos = ChunkPos::new(0, 0, 0);
        world.ensure_chunk(pos);

        assert!(MeshData::mesh_chunk(&world, pos).is_none());
    }

    #[test]
    fn unloaded_neighbor_is_air() {
        let mut world = VoxelWorld::new();

        let pos = ChunkPos::new(0, 0, 0);
        world.set_block(15, 0, 0, 1);

        let mesh = MeshData::mesh_chunk(&world, pos).unwrap();

        assert_eq!(mesh.indices.len(), 36);
    }

    #[test]
    fn loaded_neighbor_hides_shared_face() {
        let mut world = VoxelWorld::new();

        let left = ChunkPos::new(0, 0, 0);
        let right = ChunkPos::new(1, 0, 0);

        world.set_block(15, 0, 0, 1);
        world.set_block(16, 0, 0, 1);

        let left_mesh = MeshData::mesh_chunk(&world, left).unwrap();
        let right_mesh = MeshData::mesh_chunk(&world, right).unwrap();

        assert_eq!(left_mesh.indices.len(), 30);
        assert_eq!(right_mesh.indices.len(), 30);
    }

    #[test]
    fn negative_chunk_coordinates_work() {
        let mut world = VoxelWorld::new();

        let pos = ChunkPos::new(-1, 0, 0);
        world.set_block(-1, 0, 0, 5);

        let mesh = MeshData::mesh_chunk(&world, pos).unwrap();

        assert_eq!(mesh.positions.len(), 72);
        assert_eq!(mesh.block_ids.len(), 24);
        assert_eq!(mesh.indices.len(), 36);

        assert!(mesh.positions.contains(&-1.0));
        assert!(mesh.positions.contains(&0.0));
    }
}
