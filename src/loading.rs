use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_asset_ron::RonAssetPlugin;

use crate::{board::Hex, GameState};
pub mod hex;
pub mod hexes;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<Hex>::new(&["hex"]));

        AssetLoader::new(GameState::Loading)
            .with_collection::<hexes::HexAssets>()
            .with_collection::<hexes::HexImageAssets>()
            .with_collection::<hex::HexDescriptions>()
            .continue_to_state(GameState::Playing)
            .build(app);
    }
}
