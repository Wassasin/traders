use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    input::{is_close_requested, is_key_down, InputEvent, ScrollDirection},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    winit::{MouseButton, VirtualKeyCode},
};
use log::info;

use crate::entities::*;
use crate::resources::*;

pub const ARENA_HEIGHT: f32 = 1000.0;
pub const ARENA_WIDTH: f32 = 1000.0;

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(1024., 768.))
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

        create_station(world, Position::new(Point2::new(800., 700.)));
        create_station(world, Position::new(Point2::new(300., 100.)));
        create_station(world, Position::new(Point2::new(200., 600.)));

        create_trader(
            world,
            Position::new(Point2::new(900., 900.)),
            ShipBehaviour::Idle,
        );

        let t1 = create_trader(
            world,
            Position::new(Point2::new(50., 50.)),
            ShipBehaviour::Idle,
        );

        let camera_state = CameraState {
            zoom: 1.0,
            behaviour: CameraBehaviour::Follow(t1),
        };

        world.insert(camera_state);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let StateData { world, .. } = data;

        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                info!(
                    "[HANDLE_EVENT] You just interacted with a ui element: {:?}",
                    ui_event
                );
                Trans::None
            }
            StateEvent::Input(input) => {
                use InputEvent::*;
                match input {
                    MouseWheelMoved(dir) => {
                        let dir = match *dir {
                            ScrollDirection::ScrollUp => -0.1,
                            ScrollDirection::ScrollDown => 0.1,
                            _ => 0.0,
                        };
                        let mut camera_state = world.fetch_mut::<CameraState>();
                        camera_state.zoom = f32::min(f32::max(0.1, camera_state.zoom + dir), 2.0);
                    }
                    MouseButtonPressed(MouseButton::Left) => {
                        info!("Input Event detected: {:?}.", input);
                    }
                    MouseMoved { .. } | CursorMoved { .. } => (),
                    _ => {
                        info!("Input Event detected: {:?}.", input);
                    }
                }
                Trans::None
            }
        }
    }
}
