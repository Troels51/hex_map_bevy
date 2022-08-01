use crate::{board::Hex, GameState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_common_assets::ron::RonAssetPlugin;
pub mod hex_descriptions;
pub mod hex_models;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<Hex>::new(&["hex"]));
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Playing)
                .with_collection::<hex_models::HexImageAssets>()
                .with_collection::<hex_descriptions::HexDescriptions>(),
        );
    }
}
