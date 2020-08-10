use amethyst::{
    core::math,
    ecs::{Component, DenseVecStorage, VecStorage},
};
use derive_more::{Add, Deref, DerefMut, Mul, Sub};

pub type Point2 = math::geometry::Point2<f32>;
pub type Translation2 = math::geometry::Translation2<f32>;

// Note: arithmatics directly on positions do not make sense. Hence first deref.
#[derive(Deref, DerefMut, Clone, Copy, Debug)]
pub struct Position(Point2);

impl Component for Position {
    type Storage = DenseVecStorage<Self>;
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

impl Velocity {
    pub fn new(vec: Translation2) -> Self {
        Self(vec)
    }
}

#[derive(Deref, DerefMut, Clone, Copy, Debug)]
pub struct Hitbox(Translation2);

impl Component for Hitbox {
    type Storage = VecStorage<Self>;
}

impl Default for Hitbox {
    fn default() -> Self {
        Self(Translation2::new(0., 0.))
    }
}

impl Hitbox {
    pub fn new(vec: Translation2) -> Self {
        Self(vec)
    }
}

#[derive(Deref, DerefMut, Clone, Copy, Debug, Default, Sub, Mul, Add)]
pub struct Angle(f32);

impl Component for Angle {
    type Storage = VecStorage<Self>;
}

impl Angle {
    pub fn new(inner: f32) -> Self {
        Self(inner)
    }
}

#[derive(Deref, DerefMut, Clone, Copy, Debug, Default, Sub, Mul, Add)]
pub struct AngularMomentum(f32);

impl Component for AngularMomentum {
    type Storage = VecStorage<Self>;
}

impl AngularMomentum {
    pub fn new(inner: f32) -> Self {
        Self(inner)
    }
}
