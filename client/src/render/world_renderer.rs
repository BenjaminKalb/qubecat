use std::collections::HashMap;

use engine::Mat4;
use engine::mesher::MeshData;
use engine::voxels::chunk::ChunkPos;
use engine::voxels::voxel_world::VoxelWorld;

use crate::render::mesh::{BLOCK_COLORS, GpuMesh};
use crate::render::shader::ShaderProgram;

/// Кэш мешей чанков: синхронизирует dirty-чанки с GPU.
pub struct WorldRenderer {
    meshes: HashMap<ChunkPos, GpuMesh>,
}

impl WorldRenderer {
    pub fn new() -> Self {
        Self {
            meshes: HashMap::new(),
        }
    }

    /// Перемешивает dirty-чанки и убирает ставшие пустыми меши.
    /// Вызывать ПОСЛЕ app.step(), до рендера кадра.
    pub fn sync(&mut self, gl: &glow::Context, voxels: &mut VoxelWorld) {
        let dirty: Vec<ChunkPos> = voxels.dirty_chunks().copied().collect();

        for pos in dirty {
            match MeshData::mesh_chunk(voxels, pos) {
                Some(mesh) => {
                    let gpu = GpuMesh::upload(gl, &mesh).expect("загрузка меша в GPU");
                    self.meshes.insert(pos, gpu);
                }
                None => {
                    // Чанк стал пустым: убираем старый меш, если он был.
                    if let Some(old) = self.meshes.remove(&pos) {
                        old.dispose(gl);
                    }
                }
            }

            // Флаг снимается только после успешного мешинга.
            voxels
                .chunk_mut(pos)
                .expect("dirty-чанк исчез во время мешинга")
                .consume_dirty();
        }
    }

    /// Рендер всех мешей: bind → uniforms → draw.
    pub fn render(&self, gl: &glow::Context, program: &ShaderProgram, view_proj: &Mat4) {
        program.bind(gl);
        program.set_mat4(gl, "u_view_proj", view_proj);
        program.set_vec3_array(gl, "u_block_colors", &BLOCK_COLORS);

        for mesh in self.meshes.values() {
            mesh.draw(gl);
        }
    }
}

impl Default for WorldRenderer {
    fn default() -> Self {
        Self::new()
    }
}
