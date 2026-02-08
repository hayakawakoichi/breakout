use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::render::texture::ImagePlugin;

mod components;
mod constants;
mod plugins;
mod resources;
mod states;
mod storage;
mod systems;
mod utils;

#[cfg(test)]
mod test_helpers;

use constants::*;
use plugins::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "ブロック崩し".to_string(),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        resizable: false,
                        canvas: Some("#bevy_canvas".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((
            CorePlugin,
            MenuPlugin,
            GameplayPlugin,
            GameOverPlugin,
            LevelClearPlugin,
        ))
        .run();
}
