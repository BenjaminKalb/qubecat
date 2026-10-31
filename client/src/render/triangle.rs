use bytemuck::cast_slice;
use glow::HasContext as _;

/// Демо-треугольник
pub struct Triangle {
    vao: glow::VertexArray,
}

const VERTICES: [f32; 9] = [
    -0.5, -0.5, 0.0, // левый низ
    0.5, -0.5, 0.0, // правый низ
    0.0, 0.5, 0.0, // верх
];

impl Triangle {
    pub fn new(gl: &glow::Context) -> Self {
        unsafe {
            let vao = gl.create_vertex_array().expect("create_vertex_array");
            let vbo = gl.create_buffer().expect("create_buffer");

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, cast_slice(&VERTICES), glow::STATIC_DRAW);

            // attribute location 0: vec3 position
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 0, 0);

            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);

            Self { vao }
        }
    }

    pub fn draw(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_vertex_array(Some(self.vao));
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
            gl.bind_vertex_array(None);
        }
    }
}
