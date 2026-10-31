use glam::{Mat4, Vec3, camera};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub fov_y: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 10.0, 0.0),
            yaw: 0.0,
            pitch: -45.0_f32.to_radians(), // смотрим вниз на плоское поле
            fov_y: 70.0_f32.to_radians(),
            aspect: 16.0 / 9.0,
            near: 0.1,
            far: 200.0,
        }
    }
}

impl Camera {
    pub fn forward(&self) -> Vec3 {
        Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize()
    }

    pub fn right(&self) -> Vec3 {
        self.forward().cross(Vec3::Y).normalize()
    }

    pub fn rotate(&mut self, delta_x: f32, delta_y: f32, sensitivity: f32) {
        self.yaw += delta_x * sensitivity;
        self.pitch += delta_y * sensitivity;

        // Ограничиваем взгляд вверх/вниз до ~89 градусов, чтобы избежать переворота (Gimbal Lock)
        let max_pitch = 89.0_f32.to_radians();
        self.pitch = self.pitch.clamp(-max_pitch, max_pitch);
    }

    /// Матрица вида: смотрим из position вдоль forward().
    pub fn view_matrix(&self) -> Mat4 {
        let target = self.position + self.forward();
        camera::rh::view::look_at_mat4(self.position, target, Vec3::Y)
    }

    /// Проекция для OpenGL: глубина от -1 до 1.
    pub fn projection_matrix(&self) -> Mat4 {
        camera::rh::proj::opengl::perspective(self.fov_y, self.aspect, self.near, self.far)
    }

    /// Готовая связка для uniform: u_view_proj.
    pub fn view_projection(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }
}
