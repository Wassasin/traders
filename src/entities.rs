use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::{
    join::Join, Builder, Component, DispatcherBuilder, Entity, NullStorage, ReadStorage, System,
    VecStorage, World, WorldExt, WriteStorage,
};
use amethyst::renderer::{
    Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
};

pub type Coordinate = fixed::types::I32F32;
pub type AngleRadian = fixed::types::I32F32;

#[derive(Debug)]
pub struct Position {
    pub x: Coordinate,
    pub y: Coordinate,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Default)]
pub struct Velocity {
    pub x: Coordinate,
    pub y: Coordinate,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Default)]
pub struct Angle(pub AngleRadian);

impl Component for Angle {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Default)]
pub struct AngularMomentum(pub AngleRadian);

impl Component for AngularMomentum {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct Station;

impl Component for Station {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct Trader;

impl Component for Trader {
    type Storage = NullStorage<Self>;
}

pub fn create_station(world: &mut World, pos: Position) -> Entity {
    let sprite_sheet = (*world.fetch::<Handle<SpriteSheet>>()).clone();
    let spriterender = SpriteRender {
        sprite_sheet,
        sprite_number: 1,
    };
    let mut transform = Transform::default();
    transform.set_rotation_2d(0.1);
    world
        .create_entity()
        .with(Station)
        .with(pos)
        .with(spriterender)
        .with(transform)
        .with(Angle(AngleRadian::default()))
        .with(AngularMomentum(AngleRadian::from_num(0.001)))
        .build()
}

pub fn create_trader(world: &mut World, pos: Position) -> Entity {
    let sprite_sheet = (*world.fetch::<Handle<SpriteSheet>>()).clone();
    let spriterender = SpriteRender {
        sprite_sheet,
        sprite_number: 0,
    };
    world
        .create_entity()
        .with(Trader)
        .with(pos)
        .with(Velocity::default())
        .with(spriterender)
        .with(Transform::default())
        .build()
}
