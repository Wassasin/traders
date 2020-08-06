use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::entities::*;

pub const ARENA_HEIGHT: f32 = 1000.0;
pub const ARENA_WIDTH: f32 = 1000.0;

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/ships.gif",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/ships.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

pub struct Game;

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);
        world.insert(sprite_sheet_handle);

        initialise_camera(world);

        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Trader>();
        world.register::<Station>();
        world.register::<ShipBehaviour>();

        create_station(
            world,
            Position::new(Vec2 {
                x: Coordinate::from_num(800.),
                y: Coordinate::from_num(700.),
            }),
        );
        let s2 = create_station(
            world,
            Position::new(Vec2 {
                x: Coordinate::from_num(300.),
                y: Coordinate::from_num(100.),
            }),
        );

        create_trader(
            world,
            Position::new(Vec2 {
                x: Coordinate::from_num(900.),
                y: Coordinate::from_num(900.),
            }),
            ShipBehaviour::Idle
        );

        create_trader(
            world,
            Position::new(Vec2 {
                x: Coordinate::from_num(50.),
                y: Coordinate::from_num(50.),
            }),
            ShipBehaviour::FlyTo(s2)
        );
    }
}
