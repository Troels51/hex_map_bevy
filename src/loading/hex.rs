use bevy::asset::HandleUntyped;
use bevy::ecs::world::World;
use bevy::prelude::{AssetServer, Component, Handle};
use bevy_asset_loader::AssetCollection;
use serde::{Deserialize, Serialize};

use crate::board::Hex;

#[derive(Component, Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Angle(hex2d::Angle);

//We specify sides using the numbers 0-5
//starting at the top side

#[derive(AssetCollection)]
pub struct HexDescriptions {
    #[asset(path = "hexes", folder(typed))]
    pub hexes: Vec<Handle<Hex>>,
}
