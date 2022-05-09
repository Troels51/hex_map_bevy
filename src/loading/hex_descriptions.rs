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

// Loading a folder doesnt work for web. Maybe we need to figure out a way to get both?
#[derive(AssetCollection)]
pub struct HexDescriptions {
    //#[asset(path = "hexes/blank.hex")]
    //pub bland: Handle<Hex>,
    #[asset(path = "hexes/A001.hex")]
    pub a001: Handle<Hex>,
    #[asset(path = "hexes/A002.hex")]
    pub a002: Handle<Hex>,
    #[asset(path = "hexes/A003.hex")]
    pub a003: Handle<Hex>,
    #[asset(path = "hexes/A004.hex")]
    pub a004: Handle<Hex>,
    #[asset(path = "hexes/A005.hex")]
    pub a005: Handle<Hex>,
    #[asset(path = "hexes/A006.hex")]
    pub a006: Handle<Hex>,
    #[asset(path = "hexes/A007.hex")]
    pub a007: Handle<Hex>,
    #[asset(path = "hexes/A008.hex")]
    pub a008: Handle<Hex>,
    #[asset(path = "hexes/A009.hex")]
    pub a009: Handle<Hex>,
    #[asset(path = "hexes/A010.hex")]
    pub a010: Handle<Hex>,
    #[asset(path = "hexes/A011.hex")]
    pub a011: Handle<Hex>,
    #[asset(path = "hexes/A012.hex")]
    pub a012: Handle<Hex>,
    #[asset(path = "hexes/A013.hex")]
    pub a013: Handle<Hex>,
    #[asset(path = "hexes/A014.hex")]
    pub a014: Handle<Hex>,
    #[asset(path = "hexes/A015.hex")]
    pub a015: Handle<Hex>,
    #[asset(path = "hexes/A016.hex")]
    pub a016: Handle<Hex>,
    #[asset(path = "hexes/A017.hex")]
    pub a017: Handle<Hex>,
    #[asset(path = "hexes/A018.hex")]
    pub a018: Handle<Hex>,
    #[asset(path = "hexes/A019.hex")]
    pub a019: Handle<Hex>,
    #[asset(path = "hexes/A020.hex")]
    pub a020: Handle<Hex>,
    #[asset(path = "hexes/A021.hex")]
    pub a021: Handle<Hex>,
    #[asset(path = "hexes/A022.hex")]
    pub a022: Handle<Hex>,
    #[asset(path = "hexes/A023.hex")]
    pub a023: Handle<Hex>,
    #[asset(path = "hexes/A024.hex")]
    pub a024: Handle<Hex>,
    #[asset(path = "hexes/A025.hex")]
    pub a025: Handle<Hex>,
    #[asset(path = "hexes/A026.hex")]
    pub a026: Handle<Hex>,
    /*
    #[asset(path = "hexes/C001.hex")]
    pub c001: Handle<Hex>,
    #[asset(path = "hexes/C002.hex")]
    pub c002: Handle<Hex>,
    #[asset(path = "hexes/C003.hex")]
    pub c003: Handle<Hex>,
    #[asset(path = "hexes/C004.hex")]
    pub c004: Handle<Hex>,
    #[asset(path = "hexes/C005.hex")]
    pub c005: Handle<Hex>,
    #[asset(path = "hexes/C006.hex")]
    pub c006: Handle<Hex>,
    #[asset(path = "hexes/C007.hex")]
    pub c007: Handle<Hex>,
    #[asset(path = "hexes/C008.hex")]
    pub c008: Handle<Hex>,
    #[asset(path = "hexes/D001.hex")]
    pub d001: Handle<Hex>,
    #[asset(path = "hexes/D002.hex")]
    pub d002: Handle<Hex>,
    #[asset(path = "hexes/D003.hex")]
    pub d003: Handle<Hex>,
    #[asset(path = "hexes/D004.hex")]
    pub d004: Handle<Hex>,
    #[asset(path = "hexes/D005.hex")]
    pub d005: Handle<Hex>,
    #[asset(path = "hexes/D006.hex")]
    pub d006: Handle<Hex>,
    #[asset(path = "hexes/D007.hex")]
    pub d007: Handle<Hex>,
    #[asset(path = "hexes/D008.hex")]
    pub d008: Handle<Hex>,
    #[asset(path = "hexes/D009.hex")]
    pub d009: Handle<Hex>,
    #[asset(path = "hexes/D010.hex")]
    pub d010: Handle<Hex>,
    #[asset(path = "hexes/D011.hex")]
    pub d011: Handle<Hex>,
    #[asset(path = "hexes/D012.hex")]
    pub d012: Handle<Hex>,
    #[asset(path = "hexes/D013.hex")]
    pub d013: Handle<Hex>,
    #[asset(path = "hexes/M001.hex")]
    pub m001: Handle<Hex>,
    #[asset(path = "hexes/M002.hex")]
    pub m002: Handle<Hex>,
    #[asset(path = "hexes/M003.hex")]
    pub m003: Handle<Hex>,
    #[asset(path = "hexes/M004.hex")]
    pub m004: Handle<Hex>,
    #[asset(path = "hexes/M005.hex")]
    pub m005: Handle<Hex>,
    #[asset(path = "hexes/M006.hex")]
    pub m006: Handle<Hex>,
    #[asset(path = "hexes/M007.hex")]
    pub m007: Handle<Hex>,
    #[asset(path = "hexes/M008.hex")]
    pub m008: Handle<Hex>,
    #[asset(path = "hexes/M009.hex")]
    pub m009: Handle<Hex>,
    #[asset(path = "hexes/M010.hex")]
    pub m010: Handle<Hex>,
    */
}

