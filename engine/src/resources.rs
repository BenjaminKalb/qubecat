use crate::{input::Input, math::camera::Camera, time::Time};

/// Ресурсы: всё игровое состояние вне мира сущностей.
pub struct Resources {
    pub time: Time,
    pub input: Input,
    pub camera: Camera,
}
