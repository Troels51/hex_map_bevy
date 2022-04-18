use std::ops::Index;

use bevy::{core::FixedTimestep, ecs::schedule::SystemSet, prelude::*};
use bevy_asset_loader::{AssetCollection, AssetLoader};

#[derive(AssetCollection)]
pub struct HexAssets {
    #[asset(path = "models/bland_hex.glb#Scene0")]
    pub bland: Handle<Scene>,
    #[asset(path = "models/generated/A001.glb#Scene0")]
    pub a001: Handle<Scene>,
    #[asset(path = "models/generated/A002.glb#Scene0")]
    pub a002: Handle<Scene>,
    #[asset(path = "models/generated/A003.glb#Scene0")]
    pub a003: Handle<Scene>,
    #[asset(path = "models/generated/A004.glb#Scene0")]
    pub a004: Handle<Scene>,
    #[asset(path = "models/generated/A005.glb#Scene0")]
    pub a005: Handle<Scene>,
    #[asset(path = "models/generated/A006.glb#Scene0")]
    pub a006: Handle<Scene>,
    #[asset(path = "models/generated/A007.glb#Scene0")]
    pub a007: Handle<Scene>,
    #[asset(path = "models/generated/A008.glb#Scene0")]
    pub a008: Handle<Scene>,
    #[asset(path = "models/generated/A009.glb#Scene0")]
    pub a009: Handle<Scene>,
    #[asset(path = "models/generated/A010.glb#Scene0")]
    pub a010: Handle<Scene>,
    #[asset(path = "models/generated/A011.glb#Scene0")]
    pub a011: Handle<Scene>,
    #[asset(path = "models/generated/A012.glb#Scene0")]
    pub a012: Handle<Scene>,
    #[asset(path = "models/generated/A013.glb#Scene0")]
    pub a013: Handle<Scene>,
    #[asset(path = "models/generated/A014.glb#Scene0")]
    pub a014: Handle<Scene>,
    #[asset(path = "models/generated/A015.glb#Scene0")]
    pub a015: Handle<Scene>,
    #[asset(path = "models/generated/A016.glb#Scene0")]
    pub a016: Handle<Scene>,
    #[asset(path = "models/generated/A017.glb#Scene0")]
    pub a017: Handle<Scene>,
    #[asset(path = "models/generated/A018.glb#Scene0")]
    pub a018: Handle<Scene>,
    #[asset(path = "models/generated/A019.glb#Scene0")]
    pub a019: Handle<Scene>,
    #[asset(path = "models/generated/A020.glb#Scene0")]
    pub a020: Handle<Scene>,
    #[asset(path = "models/generated/A021.glb#Scene0")]
    pub a021: Handle<Scene>,
    #[asset(path = "models/generated/A022.glb#Scene0")]
    pub a022: Handle<Scene>,
    #[asset(path = "models/generated/A023.glb#Scene0")]
    pub a023: Handle<Scene>,
    #[asset(path = "models/generated/A024.glb#Scene0")]
    pub a024: Handle<Scene>,
    #[asset(path = "models/generated/A025.glb#Scene0")]
    pub a025: Handle<Scene>,
    #[asset(path = "models/generated/A026.glb#Scene0")]
    pub a026: Handle<Scene>,
    #[asset(path = "models/generated/C001.glb#Scene0")]
    pub c001: Handle<Scene>,
    #[asset(path = "models/generated/C002.glb#Scene0")]
    pub c002: Handle<Scene>,
    #[asset(path = "models/generated/C003.glb#Scene0")]
    pub c003: Handle<Scene>,
    #[asset(path = "models/generated/C004.glb#Scene0")]
    pub c004: Handle<Scene>,
    #[asset(path = "models/generated/C005.glb#Scene0")]
    pub c005: Handle<Scene>,
    #[asset(path = "models/generated/C006.glb#Scene0")]
    pub c006: Handle<Scene>,
    #[asset(path = "models/generated/C007.glb#Scene0")]
    pub c007: Handle<Scene>,
    #[asset(path = "models/generated/C008.glb#Scene0")]
    pub c008: Handle<Scene>,
    #[asset(path = "models/generated/d001.glb#Scene0")]
    pub d001: Handle<Scene>,
    #[asset(path = "models/generated/d002.glb#Scene0")]
    pub d002: Handle<Scene>,
    #[asset(path = "models/generated/d003.glb#Scene0")]
    pub d003: Handle<Scene>,
    #[asset(path = "models/generated/d004.glb#Scene0")]
    pub d004: Handle<Scene>,
    #[asset(path = "models/generated/d005.glb#Scene0")]
    pub d005: Handle<Scene>,
    #[asset(path = "models/generated/d006.glb#Scene0")]
    pub d006: Handle<Scene>,
    #[asset(path = "models/generated/d007.glb#Scene0")]
    pub d007: Handle<Scene>,
    #[asset(path = "models/generated/d008.glb#Scene0")]
    pub d008: Handle<Scene>,
    #[asset(path = "models/generated/d009.glb#Scene0")]
    pub d009: Handle<Scene>,
    #[asset(path = "models/generated/d010.glb#Scene0")]
    pub d010: Handle<Scene>,
    #[asset(path = "models/generated/d011.glb#Scene0")]
    pub d011: Handle<Scene>,
    #[asset(path = "models/generated/d012.glb#Scene0")]
    pub d012: Handle<Scene>,
    #[asset(path = "models/generated/d013.glb#Scene0")]
    pub d013: Handle<Scene>,
    #[asset(path = "models/generated/M001.glb#Scene0")]
    pub m001: Handle<Scene>,
    #[asset(path = "models/generated/M002.glb#Scene0")]
    pub m002: Handle<Scene>,
    #[asset(path = "models/generated/M003.glb#Scene0")]
    pub m003: Handle<Scene>,
    #[asset(path = "models/generated/M004.glb#Scene0")]
    pub m004: Handle<Scene>,
    #[asset(path = "models/generated/M005.glb#Scene0")]
    pub m005: Handle<Scene>,
    #[asset(path = "models/generated/M006.glb#Scene0")]
    pub m006: Handle<Scene>,
    #[asset(path = "models/generated/M007.glb#Scene0")]
    pub m007: Handle<Scene>,
    #[asset(path = "models/generated/M008.glb#Scene0")]
    pub m008: Handle<Scene>,
    #[asset(path = "models/generated/M009.glb#Scene0")]
    pub m009: Handle<Scene>,
    #[asset(path = "models/generated/M010.glb#Scene0")]
    pub m010: Handle<Scene>,
}

//Is this safe? Can it be made more safe? I like the approach of bevy Reflect
impl Index<usize> for HexAssets {
    type Output = Handle<Scene>;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.bland,
            1 => &self.a001,
            2 => &self.a002,
            3 => &self.a003,
            4 => &self.a004,
            5 => &self.a005,
            6 => &self.a006,
            7 => &self.a007,
            8 => &self.a008,
            9 => &self.a009,
            10 => &self.a010,
            11 => &self.a011,
            12 => &self.a012,
            13 => &self.a013,
            14 => &self.a014,
            15 => &self.a015,
            16 => &self.a016,
            17 => &self.a017,
            18 => &self.a018,
            19 => &self.a019,
            20 => &self.a020,
            21 => &self.a021,
            22 => &self.a022,
            23 => &self.a023,
            24 => &self.a024,
            25 => &self.a025,
            26 => &self.a026,
            27 => &self.c001,
            28 => &self.c002,
            29 => &self.c003,
            30 => &self.c004,
            31 => &self.c005,
            32 => &self.c006,
            33 => &self.c007,
            34 => &self.c008,
            35 => &self.d001,
            36 => &self.d002,
            37 => &self.d003,
            38 => &self.d004,
            39 => &self.d005,
            40 => &self.d006,
            41 => &self.d007,
            42 => &self.d008,
            43 => &self.d009,
            44 => &self.d010,
            45 => &self.d011,
            46 => &self.d012,
            47 => &self.d013,
            48 => &self.m001,
            49 => &self.m002,
            50 => &self.m003,
            51 => &self.m004,
            52 => &self.m005,
            53 => &self.m006,
            54 => &self.m007,
            55 => &self.m008,
            56 => &self.m009,
            57 => &self.m010,
            _ => panic!(),
        }
    }
}

#[derive(AssetCollection)]
pub struct HexImageAssets {
    #[asset(path = "textures/bland.png")]
    pub bland: Handle<Image>,
    #[asset(path = "textures/A001 F.png")]
    pub a001: Handle<Image>,
    #[asset(path = "textures/A002 F.png")]
    pub a002: Handle<Image>,
    #[asset(path = "textures/A003 F.png")]
    pub a003: Handle<Image>,
    #[asset(path = "textures/A004 F.png")]
    pub a004: Handle<Image>,
    #[asset(path = "textures/A005 F.png")]
    pub a005: Handle<Image>,
    #[asset(path = "textures/A006 F.png")]
    pub a006: Handle<Image>,
    #[asset(path = "textures/A007 F.png")]
    pub a007: Handle<Image>,
    #[asset(path = "textures/A008 F.png")]
    pub a008: Handle<Image>,
    #[asset(path = "textures/A009 F.png")]
    pub a009: Handle<Image>,
    #[asset(path = "textures/A010 F.png")]
    pub a010: Handle<Image>,
    #[asset(path = "textures/A011 F.png")]
    pub a011: Handle<Image>,
    #[asset(path = "textures/A012 F.png")]
    pub a012: Handle<Image>,
    #[asset(path = "textures/A013 F.png")]
    pub a013: Handle<Image>,
    #[asset(path = "textures/A014 F.png")]
    pub a014: Handle<Image>,
    #[asset(path = "textures/A015 F.png")]
    pub a015: Handle<Image>,
    #[asset(path = "textures/A016 F.png")]
    pub a016: Handle<Image>,
    #[asset(path = "textures/A017 F.png")]
    pub a017: Handle<Image>,
    #[asset(path = "textures/A018 F.png")]
    pub a018: Handle<Image>,
    #[asset(path = "textures/A019 F.png")]
    pub a019: Handle<Image>,
    #[asset(path = "textures/A020 F.png")]
    pub a020: Handle<Image>,
    #[asset(path = "textures/A021 F.png")]
    pub a021: Handle<Image>,
    #[asset(path = "textures/A022 F.png")]
    pub a022: Handle<Image>,
    #[asset(path = "textures/A023 F.png")]
    pub a023: Handle<Image>,
    #[asset(path = "textures/A024 F.png")]
    pub a024: Handle<Image>,
    #[asset(path = "textures/A025 F.png")]
    pub a025: Handle<Image>,
    #[asset(path = "textures/A026 F.png")]
    pub a026: Handle<Image>,
    #[asset(path = "textures/C001.png")]
    pub c001: Handle<Image>,
    #[asset(path = "textures/C002.png")]
    pub c002: Handle<Image>,
    #[asset(path = "textures/C003.png")]
    pub c003: Handle<Image>,
    #[asset(path = "textures/C004.png")]
    pub c004: Handle<Image>,
    #[asset(path = "textures/C005.png")]
    pub c005: Handle<Image>,
    #[asset(path = "textures/C006.png")]
    pub c006: Handle<Image>,
    #[asset(path = "textures/C007.png")]
    pub c007: Handle<Image>,
    #[asset(path = "textures/C008.png")]
    pub c008: Handle<Image>,
    #[asset(path = "textures/d001 F.png")]
    pub d001: Handle<Image>,
    #[asset(path = "textures/d002 F.png")]
    pub d002: Handle<Image>,
    #[asset(path = "textures/d003 F.png")]
    pub d003: Handle<Image>,
    #[asset(path = "textures/d004 F.png")]
    pub d004: Handle<Image>,
    #[asset(path = "textures/d005 F.png")]
    pub d005: Handle<Image>,
    #[asset(path = "textures/d006 F.png")]
    pub d006: Handle<Image>,
    #[asset(path = "textures/d007 F.png")]
    pub d007: Handle<Image>,
    #[asset(path = "textures/d008 F.png")]
    pub d008: Handle<Image>,
    #[asset(path = "textures/d009 F.png")]
    pub d009: Handle<Image>,
    #[asset(path = "textures/d010 F.png")]
    pub d010: Handle<Image>,
    #[asset(path = "textures/d011 F.png")]
    pub d011: Handle<Image>,
    #[asset(path = "textures/d012 F.png")]
    pub d012: Handle<Image>,
    #[asset(path = "textures/d013 F.png")]
    pub d013: Handle<Image>,
    #[asset(path = "textures/M001 F.png")]
    pub m001: Handle<Image>,
    #[asset(path = "textures/M002 F.png")]
    pub m002: Handle<Image>,
    #[asset(path = "textures/M003 F.png")]
    pub m003: Handle<Image>,
    #[asset(path = "textures/M004 F.png")]
    pub m004: Handle<Image>,
    #[asset(path = "textures/M005 F.png")]
    pub m005: Handle<Image>,
    #[asset(path = "textures/M006 F.png")]
    pub m006: Handle<Image>,
    #[asset(path = "textures/M007 F.png")]
    pub m007: Handle<Image>,
    #[asset(path = "textures/M008 F.png")]
    pub m008: Handle<Image>,
    #[asset(path = "textures/M009 F.png")]
    pub m009: Handle<Image>,
    #[asset(path = "textures/M010 F.png")]
    pub m010: Handle<Image>,
}

impl Index<usize> for HexImageAssets {
    type Output = Handle<Image>;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.bland,
            1 => &self.a001,
            2 => &self.a002,
            3 => &self.a003,
            4 => &self.a004,
            5 => &self.a005,
            6 => &self.a006,
            7 => &self.a007,
            8 => &self.a008,
            9 => &self.a009,
            10 => &self.a010,
            11 => &self.a011,
            12 => &self.a012,
            13 => &self.a013,
            14 => &self.a014,
            15 => &self.a015,
            16 => &self.a016,
            17 => &self.a017,
            18 => &self.a018,
            19 => &self.a019,
            20 => &self.a020,
            21 => &self.a021,
            22 => &self.a022,
            23 => &self.a023,
            24 => &self.a024,
            25 => &self.a025,
            26 => &self.a026,
            27 => &self.c001,
            28 => &self.c002,
            29 => &self.c003,
            30 => &self.c004,
            31 => &self.c005,
            32 => &self.c006,
            33 => &self.c007,
            34 => &self.c008,
            35 => &self.d001,
            36 => &self.d002,
            37 => &self.d003,
            38 => &self.d004,
            39 => &self.d005,
            40 => &self.d006,
            41 => &self.d007,
            42 => &self.d008,
            43 => &self.d009,
            44 => &self.d010,
            45 => &self.d011,
            46 => &self.d012,
            47 => &self.d013,
            48 => &self.m001,
            49 => &self.m002,
            50 => &self.m003,
            51 => &self.m004,
            52 => &self.m005,
            53 => &self.m006,
            54 => &self.m007,
            55 => &self.m008,
            56 => &self.m009,
            57 => &self.m010,
            _ => panic!(),
        }
    }
}
