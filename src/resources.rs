use crate::entities::Translation2;
use amethyst::{core::transform::Transform, ecs::Entity};

#[derive(Debug)]
pub enum CameraBehaviour {
    Static(Transform),
    Follow(Entity),
}

#[derive(Debug)]
pub struct CameraState {
    pub zoom: f32,
    pub pan: Translation2,
    pub behaviour: CameraBehaviour,
}
