use bytemuck::cast_slice;
use engine::mesher::MeshData;
use glow::HasContext;

pub const BLOCK_COLORS: [[f32; 3]; 8] = [
    [0.0, 0.0, 0.0],    // 0: AIR
    [0.5, 0.5, 0.5],    // 1: STONE
    [0.45, 0.30, 0.18], // 2
    [0.25, 0.65, 0.25], // 3
    [0.65, 0.55, 0.35], // 4
    [0.3, 0.45, 0.7],   // 5
    [0.7, 0.7, 0.7],    // 6
    [0.8, 0.8, 0.8],    // 7
];

pub struct GpuMesh {
    vao: glow::NativeVertexArray,
    position_vbo: glow::NativeBuffer,
    block_vbo: glow::NativeBuffer,
    ebo: glow::NativeBuffer,
    index_count: i32,
}

impl GpuMesh {
    /// Загружает MeshData в GPU.
    ///
    /// Attribute 0: vec3 position
    /// Attribute 1: float block_id
    pub fn upload(gl: &glow::Context, mesh: &MeshData) -> Result<Self, String> {
        let vertex_count = mesh.positions.len() / 3;

        if !mesh.positions.len().is_multiple_of(3) {
            return Err("Invalid position buffer length".into());
        }

        if mesh.block_ids.len() != vertex_count {
            return Err("Position and block_id buffers have different sizes".into());
        }

        if mesh.indices.iter().any(|&i| i as usize >= vertex_count) {
            return Err("Mesh contains invalid indices".into());
        }

        let index_count =
            i32::try_from(mesh.indices.len()).map_err(|_| "Too many indices".to_string())?;

        unsafe {
            // Создание GPU-ресурсов.
            let vao = gl.create_vertex_array()?;
            let position_vbo = gl.create_buffer()?;
            let block_vbo = gl.create_buffer()?;
            let ebo = gl.create_buffer()?;

            gl.bind_vertex_array(Some(vao));

            // Position VBO.
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(position_vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                cast_slice(&mesh.positions),
                glow::STATIC_DRAW,
            );
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 0, 0);

            // Block ID VBO.
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(block_vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                cast_slice(&mesh.block_ids),
                glow::STATIC_DRAW,
            );
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 1, glow::FLOAT, false, 0, 0);

            // Element Buffer Object.
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                cast_slice(&mesh.indices),
                glow::STATIC_DRAW,
            );

            // Отвязываем VAO.
            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);

            Ok(Self {
                vao,
                position_vbo,
                block_vbo,
                ebo,
                index_count,
            })
        }
    }

    /// Отрисовка меша.
    pub fn draw(&self, gl: &glow::Context) {
        if self.index_count == 0 {
            return;
        }

        unsafe {
            gl.bind_vertex_array(Some(self.vao));
            gl.draw_elements(glow::TRIANGLES, self.index_count, glow::UNSIGNED_INT, 0);
            gl.bind_vertex_array(None);
        }
    }

    /// Освобождение GPU-ресурсов.
    pub fn dispose(self, gl: &glow::Context) {
        unsafe {
            gl.delete_vertex_array(self.vao);
            gl.delete_buffer(self.position_vbo);
            gl.delete_buffer(self.block_vbo);
            gl.delete_buffer(self.ebo);
        }
    }
}
