use crate::{board::Hex, GameState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_common_assets::ron::RonAssetPlugin;
pub mod hex_descriptions;
pub mod hex_models;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<Hex>::new(&["hex"]));
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Playing)
        )
        .add_collection_to_loading_state::<_, hex_models::HexImageAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, hex_descriptions::HexDescriptions>(GameState::Loading);
    }
}
