use bevy::prelude::*;
use bevy_asset_loader::AssetLoader;
use bevy_asset_ron::RonAssetPlugin;

use crate::{board::Hex, GameState};
pub mod hex_descriptions;
pub mod hex_models;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<Hex>::new(&["hex"]));

        AssetLoader::new(GameState::Loading)
            .with_collection::<hex_models::HexAssets>()
            .with_collection::<hex_models::HexImageAssets>()
            .with_collection::<hex_descriptions::HexDescriptions>()
            .continue_to_state(GameState::Playing)
            .build(app);
    }
}
