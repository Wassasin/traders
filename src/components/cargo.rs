use amethyst::ecs::{Component, VecStorage};
use derive_more::{Add, Deref, DerefMut, Sub};
use std::collections::HashMap;

#[derive(Default, Deref, DerefMut, Clone, Copy, Debug, Add, Sub, PartialEq, Eq, PartialOrd)]
pub struct CargoUnits(i64);

#[derive(Default)]
pub struct CargoCapacity(CargoUnits);

impl Component for CargoCapacity {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
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

pub struct Cargo {
    inner: HashMap<CargoType, CargoUnits>,
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
            inner: HashMap::<CargoType, CargoUnits>::default(),
            cache_total: CargoUnits(0),
            capacity,
        }
    }

    /// Adapt Cargo to possess more or less of a given CargoType.
    ///
    /// Will yield CargoError when inventory or capacity is not sufficient.
    pub fn change(&mut self, t: CargoType, amount: CargoUnits) -> Result<CargoUnits, CargoError> {
        let current_amount = self.inner.entry(t).or_insert(CargoUnits::default());

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
}
