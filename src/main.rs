pub mod entities;
pub mod game;
pub mod resources;
pub mod systems;

use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

use crate::game::Game;
use std::time::Duration;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let game_data = GameDataBuilder::default()
        .with(systems::behaviour::Idle, "behaviour_idle", &[])
        .with(systems::behaviour::FlyTo, "behaviour_fly_to", &[])
        .with(systems::Movement, "movement", &["behaviour_fly_to"])
        .with(systems::Rotation, "rotation", &["behaviour_fly_to"])
        .with(
            systems::DerivePositionalTransform,
            "derive_positional_transform",
            &["movement"],
        )
        .with(
            systems::DeriveRotationalTransform,
            "derive_rotational_transform",
            &["rotation"],
        )
        .with(systems::CameraControl, "camera_control", &["movement"])
        .with(
            systems::UiRelativePositioning,
            "ui_relative_positioning",
            &["movement"],
        )
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        // .with_system_desc(UiEventHandlerSystemDesc::default(), "ui_event_handler", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?;

    let assets_dir = app_root.join("assets");
    let mut game = Application::build(assets_dir, Game)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(10)),
            50,
        )
        .build(game_data)?;
    game.run();

    Ok(())
}
