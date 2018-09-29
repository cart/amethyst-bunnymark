extern crate amethyst;
extern crate rand;

mod bunnymark;

use amethyst::{
    core::TransformBundle,
    prelude::*,
    renderer::{
        ColorMask, DepthMode, DisplayConfig, DrawSprite, Pipeline, RenderBundle, Stage, ALPHA,
    },
    utils::fps_counter::FPSCounterBundle,
};

use bunnymark::{BunnyMark, BunnyMarkBundle};

fn main() -> Result<(), amethyst::Error> {
    // logger disabled because we don't want it to affect the benchmark
    // amethyst::start_logger(Default::default());

    let display_config_path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );

    let config = DisplayConfig::load(&display_config_path);

    let assets_dir = format!("{}/assets/", env!("CARGO_MANIFEST_DIR"));
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.1, 0.1, 0.1, 1.0], 1.0)
            .with_pass(DrawSprite::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                Some(DepthMode::LessEqualWrite),
            )),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(BunnyMarkBundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(FPSCounterBundle::default())?
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?;

    let mut game = Application::build(assets_dir, BunnyMark { camera: None })?.build(game_data)?;

    game.run();
    Ok(())
}
