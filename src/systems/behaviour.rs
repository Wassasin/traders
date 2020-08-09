use crate::components::*;
use amethyst::ecs::{
    join::Join,
    prelude::{ReadStorage, System, WriteStorage},
    Entities, Entity,
};
use std::ops::{Deref, DerefMut};

pub struct Idle;

impl<'a> System<'a> for Idle {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Station>,
        WriteStorage<'a, ShipBehaviour>,
    );

    fn run(&mut self, (entities, station, mut behaviour): Self::SystemData) {
        use rand::seq::IteratorRandom;
        let mut rng = rand::thread_rng();
        let stations: Vec<Entity> = (&entities, &station).join().map(|(e, _)| e).collect();

        for behaviour in (&mut behaviour).join() {
            if let ShipBehaviour::Idle = behaviour {
                let station = stations.iter().choose(&mut rng).unwrap();
                *behaviour = ShipBehaviour::FlyTo(*station);
            }
        }
    }
}

pub struct FlyTo;

impl<'a> System<'a> for FlyTo {
    type SystemData = (
        ReadStorage<'a, Position>,
        WriteStorage<'a, ShipBehaviour>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (pos, mut behaviour, mut vel): Self::SystemData) {
        for (&our_pos, behaviour, vel) in (&pos, &mut behaviour, &mut vel).join() {
            if let ShipBehaviour::FlyTo(target) = behaviour {
                if let Some(&target_pos) = pos.get(*target) {
                    let vec = *target_pos.deref() - *our_pos.deref();
                    let len = nalgebra_glm::length(&vec);

                    let trans = if len > 0.00001 {
                        let new_len = f32::min(1., len);
                        Translation2::from(vec.scale(new_len / len))
                    } else {
                        *behaviour = ShipBehaviour::Idle;
                        Translation2::new(0., 0.)
                    };

                    *vel.deref_mut() = trans;
                }
            }
        }
    }
}
