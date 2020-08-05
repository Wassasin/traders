use crate::entities::*;
use amethyst::{
    core::transform::Transform,
    ecs::{
        join::Join,
        prelude::{ReadStorage, System, WriteStorage},
    },
};

pub struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x;
            pos.y += vel.y;
        }
    }
}

pub struct Rotation;

impl<'a> System<'a> for Rotation {
    type SystemData = (ReadStorage<'a, AngularMomentum>, WriteStorage<'a, Angle>);

    fn run(&mut self, (momentum, mut angle): Self::SystemData) {
        for (momentum, angle) in (&momentum, &mut angle).join() {
            angle.0 += momentum.0;
        }
    }
}

pub struct DerivePositionalTransform;

impl<'a> System<'a> for DerivePositionalTransform {
    type SystemData = (ReadStorage<'a, Position>, WriteStorage<'a, Transform>);

    fn run(&mut self, (pos, mut transform): Self::SystemData) {
        use fixed::traits::FromFixed;
        for (pos, transform) in (&pos, &mut transform).join() {
            transform.set_scale([1.0, 1.0, 1.0].into());
            transform.set_translation_xyz(f32::from_fixed(pos.x), f32::from_fixed(pos.y), 0.0);
        }
    }
}

pub struct DeriveRotationalTransform;

impl<'a> System<'a> for DeriveRotationalTransform {
    type SystemData = (ReadStorage<'a, Angle>, WriteStorage<'a, Transform>);

    fn run(&mut self, (angle, mut transform): Self::SystemData) {
        use fixed::traits::FromFixed;
        for (angle, transform) in (&angle, &mut transform).join() {
            transform.set_rotation_2d(f32::from_fixed(angle.0));
        }
    }
}
