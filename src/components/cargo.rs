use derive_more::{Deref, DerefMut};

#[derive(Deref, DerefMut, Clone, Copy, Debug)]
pub struct CargoUnits(u64);

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash)]
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

pub struct Cargo {}
