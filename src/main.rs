extern crate amethyst;

use amethyst::{
    prelude::*,
    renderer::{
        DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage
    },
    utils::application_root_dir,
    core::transform::{
        TransformBundle
    },
    input::InputBundle,
    ui::{
        DrawUi, UiBundle
    }
};

mod components;
mod systems;
mod pong;

use pong::{
    Pong
};

fn app_path(file_name: &str) -> String {
    format!(
        "./{}",
        //application_root_dir(),
        file_name
    )
}

fn main() -> amethyst::Result<()> {
    // amethyst::start_logger(Default::default());

    let display_config_path = app_path("resources/display_config.ron");
    let config = DisplayConfig::load(&display_config_path);

    let input_bindings_path = app_path("resources/bindings_config.ron");
    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(input_bindings_path)?;

    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 1.0], 10.0)
                .with_pass(DrawFlat2D::new())
                .with_pass(DrawUi::new())
    );

    let render_bundle = RenderBundle::new(pipe, Some(config))
        .with_sprite_sheet_processor();

    let game_data = GameDataBuilder::default()
            .with_bundle(render_bundle)?
            .with_bundle(TransformBundle::new())?
            .with_bundle(input_bundle)?
            .with_bundle(UiBundle::<String, String>::new())?
            .with(systems::PaddleSystem, "paddle_system", &["input_system"])
            .with(systems::MoveBallSystem, "move_ball_system", &[])
            .with(systems::BounceSystem, "bounce_system", &["paddle_system", "move_ball_system"])
            .with(systems::WinnerSystem, "winner_system", &["move_ball_system"]);

    let mut game = Application::new("./", Pong, game_data)?;

    game.run();

    Ok(())
}
