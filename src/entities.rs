use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::{Builder, Component, Entity, NullStorage, VecStorage, World, WorldExt},
    renderer::{SpriteRender, SpriteSheet},
};
use derive_more::{Add, Deref, DerefMut, Mul, Sub};

pub type Point2 = nalgebra::geometry::Point2<f32>;
pub type Translation2 = nalgebra::geometry::Translation2<f32>;

// Note: arithmatics directly on positions do not make sense. Hence first deref.
#[derive(Deref, DerefMut, Clone, Copy, Debug)]
pub struct Position(Point2);

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl Position {
    pub fn new(p: Point2) -> Self {
        Self(p)
    }
}

#[derive(Deref, DerefMut, Clone, Copy, Debug)]
pub struct Velocity(Translation2);

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

impl Default for Velocity {
    fn default() -> Self {
        Self(Translation2::new(0., 0.))
    }
}

#[derive(Deref, DerefMut, Clone, Copy, Debug, Default, Sub, Mul, Add)]
pub struct Angle(f32);

impl Component for Angle {
    type Storage = VecStorage<Self>;
}

#[derive(Deref, DerefMut, Clone, Copy, Debug, Default, Sub, Mul, Add)]
pub struct AngularMomentum(f32);

impl Component for AngularMomentum {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub enum ShipBehaviour {
    Idle,
    FlyTo(Entity),
}

impl Component for ShipBehaviour {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct Station;

impl Component for Station {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct Trader;

impl Component for Trader {
    type Storage = NullStorage<Self>;
}

pub fn create_station(world: &mut World, pos: Position) -> Entity {
    let sprite_sheet = (*world.fetch::<Handle<SpriteSheet>>()).clone();
    let spriterender = SpriteRender {
        sprite_sheet,
        sprite_number: 1,
    };
    world
        .create_entity()
        .with(Station)
        .with(pos)
        .with(spriterender)
        .with(Transform::default())
        .with(Angle(f32::default()))
        .with(AngularMomentum(0.001))
        .build()
}

pub fn create_trader(world: &mut World, pos: Position, behaviour: ShipBehaviour) -> Entity {
    let sprite_sheet = (*world.fetch::<Handle<SpriteSheet>>()).clone();
    let spriterender = SpriteRender {
        sprite_sheet,
        sprite_number: 0,
    };
    world
        .create_entity()
        .with(Trader)
        .with(pos)
        .with(Velocity::default())
        .with(spriterender)
        .with(Transform::default())
        .with(behaviour)
        .build()
}
