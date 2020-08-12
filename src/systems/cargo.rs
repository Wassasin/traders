use crate::components::*;
use amethyst::ecs::{
    join::Join,
    prelude::{ReadExpect, System, WriteStorage},
};

use crate::resources::*;

pub struct Fabrication;

impl<'a> System<'a> for Fabrication {
    type SystemData = (
        ReadExpect<'a, CurrentTime>,
        WriteStorage<'a, Cargo>,
        WriteStorage<'a, FabricationModule>,
    );

    fn run(&mut self, (current_time, mut cargo, mut fabrication_module): Self::SystemData) {
        for (cargo, fabrication_module) in (&mut cargo, &mut fabrication_module).join() {
            use FabricationState::*;
            let FabricationModule { state, recipe } = fabrication_module;

            // Try to finish up production if capable.
            let finish_production = match state {
                Idle => false,
                Hanging => true,
                Running { deadline } if *deadline <= current_time.0 => true,
                Running { .. } => false,
            };

            if finish_production {
                // Adds products to cargo.
                match cargo.mass_change_iter(recipe.products.iter().map(|t| *t)) {
                    Ok(()) => *state = Idle,
                    Err(_) => *state = Hanging,
                }
            }

            if let Idle = state {
                // Removes cargo from ingredients list.
                if cargo
                    .mass_change_iter(
                        recipe
                            .ingredients
                            .iter()
                            .map(|(k, CargoUnits(v))| (*k, CargoUnits(-v))),
                    )
                    .is_ok()
                {
                    *state = Running {
                        deadline: current_time.0 + recipe.duration,
                    };
                }
            }
        }
    }
}
