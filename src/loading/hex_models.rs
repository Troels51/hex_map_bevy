use std::ops::Index;

use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
pub struct HexImageAssets {
    #[asset(path = "textures/blank.png")]
    pub blank: Handle<Image>,
    #[asset(path = "textures/A001.png")]
    pub a001: Handle<Image>,
    #[asset(path = "textures/A002.png")]
    pub a002: Handle<Image>,
    #[asset(path = "textures/A003.png")]
    pub a003: Handle<Image>,
    #[asset(path = "textures/A004.png")]
    pub a004: Handle<Image>,
    #[asset(path = "textures/A005.png")]
    pub a005: Handle<Image>,
    #[asset(path = "textures/A006.png")]
    pub a006: Handle<Image>,
    #[asset(path = "textures/A007.png")]
    pub a007: Handle<Image>,
    #[asset(path = "textures/A008.png")]
    pub a008: Handle<Image>,
    #[asset(path = "textures/A009.png")]
    pub a009: Handle<Image>,
    #[asset(path = "textures/A010.png")]
    pub a010: Handle<Image>,
    #[asset(path = "textures/A011.png")]
    pub a011: Handle<Image>,
    #[asset(path = "textures/A012.png")]
    pub a012: Handle<Image>,
    #[asset(path = "textures/A013.png")]
    pub a013: Handle<Image>,
    #[asset(path = "textures/A014.png")]
    pub a014: Handle<Image>,
    #[asset(path = "textures/A015.png")]
    pub a015: Handle<Image>,
    #[asset(path = "textures/A016.png")]
    pub a016: Handle<Image>,
    #[asset(path = "textures/A017.png")]
    pub a017: Handle<Image>,
    #[asset(path = "textures/A018.png")]
    pub a018: Handle<Image>,
    #[asset(path = "textures/A019.png")]
    pub a019: Handle<Image>,
    #[asset(path = "textures/A020.png")]
    pub a020: Handle<Image>,
    #[asset(path = "textures/A021.png")]
    pub a021: Handle<Image>,
    #[asset(path = "textures/A022.png")]
    pub a022: Handle<Image>,
    #[asset(path = "textures/A023.png")]
    pub a023: Handle<Image>,
    #[asset(path = "textures/A024.png")]
    pub a024: Handle<Image>,
    #[asset(path = "textures/A025.png")]
    pub a025: Handle<Image>,
    #[asset(path = "textures/A026.png")]
    pub a026: Handle<Image>,
    #[asset(path = "textures/A027.png")]
    pub a027: Handle<Image>,
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

    #[asset(path = "textures/Dragon1.png")]
    pub dragon1: Handle<Image>,
    #[asset(path = "textures/Dragon2.png")]
    pub dragon2: Handle<Image>,
    #[asset(path = "textures/Dragon3.png")]
    pub dragon3: Handle<Image>,
    #[asset(path = "textures/Dragon4.png")]
    pub dragon4: Handle<Image>,
    #[asset(path = "textures/Dragon5.png")]
    pub dragon5: Handle<Image>,
    #[asset(path = "textures/Dragon6.png")]
    pub dragon6: Handle<Image>,
    #[asset(path = "textures/Dragon7.png")]
    pub dragon7: Handle<Image>,

    #[asset(path = "textures/F001.png")]
    pub f001: Handle<Image>,
    #[asset(path = "textures/F002.png")]
    pub f002: Handle<Image>,
    #[asset(path = "textures/F003.png")]
    pub f003: Handle<Image>,
    #[asset(path = "textures/F004.png")]
    pub f004: Handle<Image>,
    #[asset(path = "textures/F005.png")]
    pub f005: Handle<Image>,
    #[asset(path = "textures/F006.png")]
    pub f006: Handle<Image>,
    #[asset(path = "textures/F007.png")]
    pub f007: Handle<Image>,
    #[asset(path = "textures/F008.png")]
    pub f008: Handle<Image>,
    #[asset(path = "textures/F009.png")]
    pub f009: Handle<Image>,
    #[asset(path = "textures/F010.png")]
    pub f010: Handle<Image>,

    #[asset(path = "textures/LRA01.png")]
    pub lra01: Handle<Image>,
    #[asset(path = "textures/LRA02.png")]
    pub lra02: Handle<Image>,
    #[asset(path = "textures/LRA03.png")]
    pub lra03: Handle<Image>,
    #[asset(path = "textures/LRA04.png")]
    pub lra04: Handle<Image>,
    #[asset(path = "textures/LRA05.png")]
    pub lra05: Handle<Image>,
    #[asset(path = "textures/LRA06.png")]
    pub lra06: Handle<Image>,
    #[asset(path = "textures/LRA07.png")]
    pub lra07: Handle<Image>,
    #[asset(path = "textures/LRA08.png")]
    pub lra08: Handle<Image>,

    #[asset(path = "textures/R001.png")]
    pub r001: Handle<Image>,
    #[asset(path = "textures/R002.png")]
    pub r002: Handle<Image>,
    #[asset(path = "textures/R003.png")]
    pub r003: Handle<Image>,
    #[asset(path = "textures/R004.png")]
    pub r004: Handle<Image>,
    #[asset(path = "textures/R005.png")]
    pub r005: Handle<Image>,
    #[asset(path = "textures/R006.png")]
    pub r006: Handle<Image>,
    #[asset(path = "textures/R007.png")]
    pub r007: Handle<Image>,
    #[asset(path = "textures/R008.png")]
    pub r008: Handle<Image>,
    #[asset(path = "textures/R009.png")]
    pub r009: Handle<Image>,
    #[asset(path = "textures/R010.png")]
    pub r010: Handle<Image>,
}

impl Index<usize> for HexImageAssets {
    type Output = Handle<Image>;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.blank,
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
            27 => &self.a027,

            28 => &self.c001,
            29 => &self.c002,
            30 => &self.c003,
            31 => &self.c004,
            32 => &self.c005,
            33 => &self.c006,
            34 => &self.c007,
            35 => &self.c008,

            36 => &self.dragon1,
            37 => &self.dragon2,
            38 => &self.dragon3,
            39 => &self.dragon4,
            40 => &self.dragon5,
            41 => &self.dragon6,
            42 => &self.dragon7,

            43 => &self.f001,
            44 => &self.f002,
            45 => &self.f003,
            46 => &self.f004,
            47 => &self.f005,
            48 => &self.f006,
            49 => &self.f007,
            50 => &self.f008,
            51 => &self.f009,
            52 => &self.f010,

            53 => &self.lra01,
            54 => &self.lra02,
            55 => &self.lra03,
            56 => &self.lra04,
            57 => &self.lra05,
            58 => &self.lra06,
            59 => &self.lra07,
            60 => &self.lra08,

            61 => &self.r001,
            62 => &self.r002,
            63 => &self.r003,
            64 => &self.r004,
            65 => &self.r005,
            66 => &self.r006,
            67 => &self.r007,
            68 => &self.r008,
            69 => &self.r009,
            70 => &self.r010,
            _ => panic!(),
        }
    }
}
impl HexImageAssets {
    pub fn get(&self, key: &str) -> &Handle<Image> {
        match key {
            "blank" => &self.blank,
            "A001" => &self.a001,
            "A002" => &self.a002,
            "A003" => &self.a003,
            "A004" => &self.a004,
            "A005" => &self.a005,
            "A006" => &self.a006,
            "A007" => &self.a007,
            "A008" => &self.a008,
            "A009" => &self.a009,
            "A010" => &self.a010,
            "A011" => &self.a011,
            "A012" => &self.a012,
            "A013" => &self.a013,
            "A014" => &self.a014,
            "A015" => &self.a015,
            "A016" => &self.a016,
            "A017" => &self.a017,
            "A018" => &self.a018,
            "A019" => &self.a019,
            "A020" => &self.a020,
            "A021" => &self.a021,
            "A022" => &self.a022,
            "A023" => &self.a023,
            "A024" => &self.a024,
            "A025" => &self.a025,
            "A026" => &self.a026,
            "A027" => &self.a026,

            "C001" => &self.c001,
            "C002" => &self.c002,
            "C003" => &self.c003,
            "C004" => &self.c004,
            "C005" => &self.c005,
            "C006" => &self.c006,
            "C007" => &self.c007,
            "C008" => &self.c008,

            "Dragon1" => &self.dragon1,
            "Dragon2" => &self.dragon2,
            "Dragon3" => &self.dragon3,
            "Dragon4" => &self.dragon4,
            "Dragon5" => &self.dragon5,
            "Dragon6" => &self.dragon6,
            "Dragon7" => &self.dragon7,

            "F001" => &self.f001,
            "F002" => &self.f002,
            "F003" => &self.f003,
            "F004" => &self.f004,
            "F005" => &self.f005,
            "F006" => &self.f006,
            "F007" => &self.f007,
            "F008" => &self.f008,
            "F009" => &self.f009,
            "F010" => &self.f010,

            "LRA01" => &self.lra01,
            "LRA02" => &self.lra02,
            "LRA03" => &self.lra03,
            "LRA04" => &self.lra04,
            "LRA05" => &self.lra05,
            "LRA06" => &self.lra06,
            "LRA07" => &self.lra07,
            "LRA08" => &self.lra08,

            "R001" => &self.r001,
            "R002" => &self.r002,
            "R003" => &self.r003,
            "R004" => &self.r004,
            "R005" => &self.r005,
            "R006" => &self.r006,
            "R007" => &self.r007,
            "R008" => &self.r008,
            "R009" => &self.r009,
            "R010" => &self.r010,
            _ => panic!("{}", key),
        }
    }
}
