pub mod behaviour;

use amethyst::{
    core::transform::Transform,
    ecs::{
        join::Join,
        prelude::{ReadExpect, ReadStorage, System, WriteExpect, WriteStorage},
    },
    renderer::Camera,
    window::Window,
};
use std::ops::{Deref, DerefMut};

use crate::entities::*;
use crate::resources::*;

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

pub struct DerivePositionalTransform;

impl<'a> System<'a> for DerivePositionalTransform {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Trader>,
        ReadStorage<'a, Station>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (pos, trader, station, mut transform): Self::SystemData) {
        for (pos, trader, station, transform) in
            (&pos, (&trader).maybe(), (&station).maybe(), &mut transform).join()
        {
            transform.set_scale([1.0, 1.0, 1.0].into());

            // Vary z-level based on type
            let z = if trader.is_some() {
                0.2
            } else if station.is_some() {
                0.1
            } else {
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

pub struct CameraControl;

impl<'a> System<'a> for CameraControl {
    type SystemData = (
        WriteExpect<'a, CameraState>,
        ReadExpect<'a, Window>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Camera>,
        WriteStorage<'a, Transform>,
    );

    fn run(
        &mut self,
        (mut camera_state, window, pos, mut camera, mut transform): Self::SystemData,
    ) {
        let size = window.deref().get_inner_size().unwrap();
        let zoom = camera_state.zoom;

        for (camera, transform) in (&mut camera, &mut transform).join() {
            // Update the camera size per zoom level.
            *camera = Camera::standard_2d(size.width as f32 / zoom, size.height as f32 / zoom);

            // Update the camera position.
            if camera_state.pan.x != 0. || camera_state.pan.y != 0. {
                let mut t = transform.clone();
                let factor = f32::powf(2., 1. / zoom);
                t.append_translation_xyz(
                    camera_state.pan.x * factor,
                    camera_state.pan.y * factor,
                    0.,
                );
                camera_state.behaviour = CameraBehaviour::Static(t);
            }

            match &camera_state.behaviour {
                CameraBehaviour::Static(t) => *transform = t.clone(),
                CameraBehaviour::Follow(target) => {
                    if let Some(pos) = pos.get(*target) {
                        transform.set_translation_xyz(pos.x, pos.y, 1.0);
                    }
                }
            }
        }
    }
}
