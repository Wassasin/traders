use crate::components::{Time, Translation2};
use amethyst::ecs::Entity;
use std::collections::HashSet;

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

#[derive(Debug, Default)]
pub struct Selection(pub Option<Entity>);

#[derive(Debug, Default)]
pub struct HoverSelectable(pub HashSet<Entity>);

#[derive(Debug, Default)]
pub struct CurrentTime(pub Time);
