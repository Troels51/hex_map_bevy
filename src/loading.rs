use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
pub mod hexes;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(GameState::Loading)
            .with_collection::<hexes::HexAssets>()
            .with_collection::<hexes::HexImageAssets>()
            .continue_to_state(GameState::Playing)
            .build(app);
    }
}
