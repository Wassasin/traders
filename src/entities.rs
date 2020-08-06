use amethyst::{
    assets::{Handle},
    core::transform::Transform,
    ecs::{
         Builder, Component, Entity, NullStorage,
        VecStorage, World, WorldExt,
    },
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet},
};
use std::ops::{Deref};
use derive_more::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign, Sub, SubAssign};

type FixedNum = fixed::types::I32F32;

#[derive(
    Clone, Copy, Debug, Default, Deref, DerefMut, Sub, Mul, Add, SubAssign, MulAssign, AddAssign,
)]
pub struct Coordinate(fixed::types::I32F32);

#[derive(
    Clone, Copy, Debug, Default, Deref, DerefMut, Sub, Mul, Add, SubAssign, MulAssign, AddAssign,
)]
pub struct AngleRadian(fixed::types::I32F32);

impl Coordinate {
    pub fn from_num<T: fixed::traits::ToFixed>(t: T) -> Self {
        Self(FixedNum::from_num(t))
    }
}

impl AngleRadian {
    pub fn from_num<T: fixed::traits::ToFixed>(t: T) -> Self {
        Self(FixedNum::from_num(t))
    }
}

impl Into<f32> for Coordinate {
    fn into(self) -> f32 {
        use fixed::traits::FromFixed;
        f32::from_fixed(self.0)
    }
}

impl Into<f32> for AngleRadian {
    fn into(self) -> f32 {
        use fixed::traits::FromFixed;
        f32::from_fixed(self.0)
    }
}

#[derive(Clone, Copy, Debug, Default, Sub, Mul, Add, SubAssign, MulAssign, AddAssign)]
pub struct Vec2 {
    pub x: Coordinate,
    pub y: Coordinate,
}

// Note: arithmatics directly on positions do not make sense. Hence first deref.
#[derive(Deref, DerefMut, Clone, Copy, Debug, Default)]
pub struct Position(Vec2);

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl Position {
    pub fn new(vec: Vec2) -> Self {
        Self(vec)
    }
}

#[derive(Deref, DerefMut, Clone, Copy, Debug, Default, Sub, Mul, Add)]
pub struct Velocity(Vec2);

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

#[derive(Deref, DerefMut, Clone, Copy, Debug, Default, Sub, Mul, Add)]
pub struct Angle(AngleRadian);

impl Component for Angle {
    type Storage = VecStorage<Self>;
}

impl Into<f32> for Angle {
    fn into(self) -> f32 {
        (*self.deref()).into()
    }
}

#[derive(Deref, DerefMut, Clone, Copy, Debug, Default, Sub, Mul, Add)]
pub struct AngularMomentum(AngleRadian);

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
        .with(Angle(AngleRadian::default()))
        .with(AngularMomentum(AngleRadian::from_num(0.001)))
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
