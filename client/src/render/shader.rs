use glow::HasContext;

/// Скомпилированная шейдерная программа. Компилируется из строк
pub struct ShaderProgram {
    program: glow::Program,
}

impl ShaderProgram {
    /// Паникует при ошибке компиляции/линковки
    pub fn new(gl: &glow::Context, vertex_src: &str, fragment_src: &str) -> Self {
        unsafe {
            let vs = gl
                .create_shader(glow::VERTEX_SHADER)
                .expect("create_shader vs");
            gl.shader_source(vs, vertex_src);
            gl.compile_shader(vs);
            assert_shader_ok(gl, vs, "vertex");

            let fs = gl
                .create_shader(glow::FRAGMENT_SHADER)
                .expect("create_shader fs");
            gl.shader_source(fs, fragment_src);
            gl.compile_shader(fs);
            assert_shader_ok(gl, fs, "fragment");

            let program = gl.create_program().expect("create_program");
            gl.attach_shader(program, vs);
            gl.attach_shader(program, fs);
            gl.link_program(program);
            let ok = gl.get_program_link_status(program);
            assert!(
                ok,
                "линковка программы: {}",
                gl.get_program_info_log(program)
            );

            gl.delete_shader(vs);
            gl.delete_shader(fs);

            Self { program }
        }
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(Some(self.program));
        }
    }
}

unsafe fn assert_shader_ok(gl: &glow::Context, shader: glow::Shader, kind: &str) {
    unsafe {
        let ok = gl.get_shader_compile_status(shader);
        assert!(
            ok,
            "компиляция {kind}-шейдера: {}",
            gl.get_shader_info_log(shader)
        );
    }
}
