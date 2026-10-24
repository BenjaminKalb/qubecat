use hecs::{CommandBuffer, World};

use crate::resources::Resources;
/// Тип системы: функция над миром, ресурсами и буфером команд.
pub type SystemFn = fn(
    &mut World, 
    &mut Resources, 
    &mut CommandBuffer
);