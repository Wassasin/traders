use crate::entities::Translation2;
use amethyst::ecs::Entity;

#[derive(Debug)]
pub enum CameraBehaviour {
    Static,
    Pan(Translation2),
    Follow(Entity),
}

#[derive(Debug)]
pub struct CameraState {
    pub zoom: f32,
    pub behaviour: CameraBehaviour,
}
