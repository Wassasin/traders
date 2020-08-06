use crate::entities::*;
use amethyst::{
    core::transform::Transform,
    ecs::{
        join::Join,
        prelude::{ReadStorage, System, WriteStorage},
    },
};
use std::ops::{Deref, DerefMut};

pub struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        for (vel, pos) in (&vel, &mut pos).join() {
            *pos.deref_mut() += *vel.deref();
        }
    }
}

pub struct Rotation;

impl<'a> System<'a> for Rotation {
    type SystemData = (ReadStorage<'a, AngularMomentum>, WriteStorage<'a, Angle>);

    fn run(&mut self, (momentum, mut angle): Self::SystemData) {
        for (momentum, angle) in (&momentum, &mut angle).join() {
            *angle.deref_mut() += *momentum.deref();
        }
    }
}

pub struct Navigation;

impl<'a> System<'a> for Navigation {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, ShipBehaviour>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (pos, behaviour, mut vel): Self::SystemData) {
        use crate::entities::ShipBehaviour::FlyTo;
        for (&our_pos, behaviour, vel) in (&pos, &behaviour, &mut vel).join() {
            if let FlyTo(target) = behaviour {
                if let Some(&target_pos) = pos.get(*target) {
                    let vec = *our_pos.deref() - *target_pos.deref();
                }
            }
        }
    }
}

pub struct DerivePositionalTransform;

impl<'a> System<'a> for DerivePositionalTransform {
    type SystemData = (ReadStorage<'a, Position>, WriteStorage<'a, Transform>);

    fn run(&mut self, (pos, mut transform): Self::SystemData) {
        for (pos, transform) in (&pos, &mut transform).join() {
            transform.set_scale([1.0, 1.0, 1.0].into());
            transform.set_translation_xyz(pos.x.into(), pos.y.into(), 0.0);
        }
    }
}

pub struct DeriveRotationalTransform;

impl<'a> System<'a> for DeriveRotationalTransform {
    type SystemData = (ReadStorage<'a, Angle>, WriteStorage<'a, Transform>);

    fn run(&mut self, (angle, mut transform): Self::SystemData) {
        for (angle, transform) in (&angle, &mut transform).join() {
            transform.set_rotation_2d((*angle).into());
        }
    }
}
