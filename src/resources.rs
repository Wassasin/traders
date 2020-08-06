use amethyst::{core::transform::Transform, ecs::Entity};

#[derive(Debug)]
pub enum CameraBehaviour {
    Static(Transform),
    Follow(Entity),
}
