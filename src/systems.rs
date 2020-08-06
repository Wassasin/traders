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
            *pos.deref_mut() = vel.deref().transform_point(pos.deref());
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
                    let vec = *target_pos.deref() - *our_pos.deref();
                    let len = nalgebra_glm::length(&vec);

                    let trans = if len > 0.00001 {
                        let new_len = f32::min(0.5, len);
                        Translation2::from(vec.scale(new_len / len))
                    } else {
                        Translation2::new(0., 0.)
                    };

                    *vel.deref_mut() = trans;
                }
            }
        }
    }
}

pub struct DerivePositionalTransform;

impl<'a> System<'a> for DerivePositionalTransform {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Trader>,
        ReadStorage<'a, Station>,
        WriteStorage<'a, Transform>
    );

    fn run(&mut self, (pos, trader, station, mut transform): Self::SystemData) {
        for (pos, trader, station, transform) in (&pos, (&trader).maybe(), (&station).maybe(), &mut transform).join() {
            transform.set_scale([1.0, 1.0, 1.0].into());

            // Vary z-level based on type
            let z = if trader.is_some() {
                0.2
            } else if station.is_some() {
                0.1
            }else {
                0.0
            };

            transform.set_translation_xyz(pos.x.into(), pos.y.into(), z);
        }
    }
}

pub struct DeriveRotationalTransform;

impl<'a> System<'a> for DeriveRotationalTransform {
    type SystemData = (ReadStorage<'a, Angle>, WriteStorage<'a, Transform>);

    fn run(&mut self, (angle, mut transform): Self::SystemData) {
        for (angle, transform) in (&angle, &mut transform).join() {
            transform.set_rotation_2d(*angle.deref());
        }
    }
}
