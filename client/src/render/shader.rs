use engine::Mat4;
use glow::HasContext;

pub struct ShaderProgram {
    program: glow::Program,
}

impl ShaderProgram {
    pub fn new(gl: &glow::Context, vertex_src: &str, fragment_src: &str) -> Self {
        unsafe {
            let vs = compile_shader(gl, glow::VERTEX_SHADER, vertex_src, "vertex");

            let fs = compile_shader(gl, glow::FRAGMENT_SHADER, fragment_src, "fragment");

            let program = gl.create_program().expect("create_program");

            gl.attach_shader(program, vs);
            gl.attach_shader(program, fs);
            gl.link_program(program);

            let linked = gl.get_program_link_status(program);

            gl.detach_shader(program, vs);
            gl.detach_shader(program, fs);

            gl.delete_shader(vs);
            gl.delete_shader(fs);

            assert!(
                linked,
                "Линковка программы: {}",
                gl.get_program_info_log(program)
            );

            Self { program }
        }
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(Some(self.program));
        }
    }

    pub fn set_mat4(&self, gl: &glow::Context, name: &str, value: &Mat4) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);

            if let Some(location) = location {
                gl.uniform_matrix_4_f32_slice(Some(&location), false, &value.to_cols_array());
            }
        }
    }

    pub fn set_vec3_array(&self, gl: &glow::Context, name: &str, values: &[[f32; 3]]) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);

            if let Some(location) = location {
                let flat: Vec<f32> = values.iter().flat_map(|v| v.iter().copied()).collect();

                gl.uniform_3_f32_slice(Some(&location), &flat);
            }
        }
    }

    pub unsafe fn dispose(self, gl: &glow::Context) {
        unsafe {
            gl.delete_program(self.program);
        }
    }
}

unsafe fn compile_shader(
    gl: &glow::Context,
    shader_type: u32,
    source: &str,
    kind: &str,
) -> glow::Shader {
    unsafe {
        let shader = gl
            .create_shader(shader_type)
            .unwrap_or_else(|e| panic!("Создание {kind}-шейдера: {e}"));

        gl.shader_source(shader, source);
        gl.compile_shader(shader);

        if !gl.get_shader_compile_status(shader) {
            let log = gl.get_shader_info_log(shader);
            gl.delete_shader(shader);

            panic!("Компиляция {kind}-шейдера: {log}");
        }

        shader
    }
}
