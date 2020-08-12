use amethyst::ecs::{Component, VecStorage};
use derive_more::{Add, Deref, DerefMut, Sub};
use enum_map::{Enum, EnumMap};

use super::*;

#[derive(Default, Deref, DerefMut, Clone, Copy, Debug, Add, Sub, PartialEq, Eq, PartialOrd)]
pub struct CargoUnits(pub i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Enum)]
pub enum CargoType {
    CarbonOre,
    MetalOre,
    IceOre,

    Metal,
    Carbon,
    Water,
    Nitrogen,
    Oxygen,

    Fuel,
    Food,
    Waste,

    Robotics,
    Electronics,
    Hullplating,
}

#[derive(Debug, Clone)]
pub struct Cargo {
    inner: EnumMap<CargoType, CargoUnits>,
    cache_total: CargoUnits,
    capacity: CargoUnits,
}

impl Component for Cargo {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub enum CargoError {
    Insufficient,
    OverCapacity,
}

impl Cargo {
    pub fn new(capacity: CargoUnits) -> Self {
        Cargo {
            inner: EnumMap::<CargoType, CargoUnits>::default(),
            cache_total: CargoUnits(0),
            capacity,
        }
    }

    pub fn is_full(&self) -> bool {
        self.cache_total == self.capacity
    }

    /// Adapt Cargo to possess more or less of a given CargoType.
    ///
    /// Will yield CargoError when inventory or capacity is not sufficient.
    pub fn change(&mut self, t: CargoType, amount: CargoUnits) -> Result<CargoUnits, CargoError> {
        let current_amount: &mut CargoUnits = &mut self.inner[t];

        let new_amount = *current_amount + amount;
        let new_cache_total = self.cache_total + amount;

        if new_amount < CargoUnits::default() {
            return Err(CargoError::Insufficient);
        }

        if new_cache_total > self.capacity {
            return Err(CargoError::OverCapacity);
        }

        *current_amount = new_amount;
        self.cache_total = new_cache_total;
        Ok(new_amount)
    }

    pub fn mass_change_iter(
        &mut self,
        other: impl Iterator<Item = (CargoType, CargoUnits)>,
    ) -> Result<(), CargoError> {
        let mut copy = self.clone();
        for (k, v) in other {
            if v != CargoUnits(0) {
                copy.change(k, v)?;
            }
        }
        *self = copy;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct FabricationRecipe {
    pub duration: Time,
    pub ingredients: &'static [(CargoType, CargoUnits)],
    pub products: &'static [(CargoType, CargoUnits)],
}

#[derive(Clone, Debug)]
pub enum FabricationState {
    /// Fabrication is not running.
    Idle,
    /// Fabrication is done, but awaiting delivery (due to OverCapacity)
    Hanging,
    /// Fabrication is running, and will be done on the deadline.
    Running { deadline: Time },
}

#[derive(Clone, Debug)]
pub struct FabricationModule {
    pub state: FabricationState,
    pub recipe: &'static FabricationRecipe,
}

impl Component for FabricationModule {
    type Storage = VecStorage<Self>;
}

pub static METAL_ORE_RECIPE: FabricationRecipe = FabricationRecipe {
    duration: Time(50),
    ingredients: &[],
    products: &[(CargoType::MetalOre, CargoUnits(1))],
};
pub static METAL_RECIPE: FabricationRecipe = FabricationRecipe {
    duration: Time(100),
    ingredients: &[(CargoType::MetalOre, CargoUnits(5))],
    products: &[(CargoType::Metal, CargoUnits(1))],
};
