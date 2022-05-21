use std::ops::Index;

use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
pub struct HexAssets {
    #[asset(path = "models/blank.glb#Scene0")]
    pub blank: Handle<Scene>,
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
    #[asset(path = "models/generated/A027.glb#Scene0")]
    pub a027: Handle<Scene>,

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

    #[asset(path = "models/generated/Dragon1.glb#Scene0")]
    pub dragon1: Handle<Scene>,
    #[asset(path = "models/generated/Dragon2.glb#Scene0")]
    pub dragon2: Handle<Scene>,
    #[asset(path = "models/generated/Dragon3.glb#Scene0")]
    pub dragon3: Handle<Scene>,
    #[asset(path = "models/generated/Dragon4.glb#Scene0")]
    pub dragon4: Handle<Scene>,
    #[asset(path = "models/generated/Dragon5.glb#Scene0")]
    pub dragon5: Handle<Scene>,
    #[asset(path = "models/generated/Dragon6.glb#Scene0")]
    pub dragon6: Handle<Scene>,
    #[asset(path = "models/generated/Dragon7.glb#Scene0")]
    pub dragon7: Handle<Scene>,

    #[asset(path = "models/generated/F001.glb#Scene0")]
    pub f001: Handle<Scene>,
    #[asset(path = "models/generated/F002.glb#Scene0")]
    pub f002: Handle<Scene>,
    #[asset(path = "models/generated/F003.glb#Scene0")]
    pub f003: Handle<Scene>,
    #[asset(path = "models/generated/F004.glb#Scene0")]
    pub f004: Handle<Scene>,
    #[asset(path = "models/generated/F005.glb#Scene0")]
    pub f005: Handle<Scene>,
    #[asset(path = "models/generated/F006.glb#Scene0")]
    pub f006: Handle<Scene>,
    #[asset(path = "models/generated/F007.glb#Scene0")]
    pub f007: Handle<Scene>,
    #[asset(path = "models/generated/F008.glb#Scene0")]
    pub f008: Handle<Scene>,
    #[asset(path = "models/generated/F009.glb#Scene0")]
    pub f009: Handle<Scene>,
    #[asset(path = "models/generated/F010.glb#Scene0")]
    pub f010: Handle<Scene>,

    #[asset(path = "models/generated/LRA01.glb#Scene0")]
    pub lra01: Handle<Scene>,
    #[asset(path = "models/generated/LRA02.glb#Scene0")]
    pub lra02: Handle<Scene>,
    #[asset(path = "models/generated/LRA03.glb#Scene0")]
    pub lra03: Handle<Scene>,
    #[asset(path = "models/generated/LRA04.glb#Scene0")]
    pub lra04: Handle<Scene>,
    #[asset(path = "models/generated/LRA05.glb#Scene0")]
    pub lra05: Handle<Scene>,
    #[asset(path = "models/generated/LRA06.glb#Scene0")]
    pub lra06: Handle<Scene>,
    #[asset(path = "models/generated/LRA07.glb#Scene0")]
    pub lra07: Handle<Scene>,
    #[asset(path = "models/generated/LRA08.glb#Scene0")]
    pub lra08: Handle<Scene>,

    #[asset(path = "models/generated/R001.glb#Scene0")]
    pub r001: Handle<Scene>,
    #[asset(path = "models/generated/R002.glb#Scene0")]
    pub r002: Handle<Scene>,
    #[asset(path = "models/generated/R003.glb#Scene0")]
    pub r003: Handle<Scene>,
    #[asset(path = "models/generated/R004.glb#Scene0")]
    pub r004: Handle<Scene>,
    #[asset(path = "models/generated/R005.glb#Scene0")]
    pub r005: Handle<Scene>,
    #[asset(path = "models/generated/R006.glb#Scene0")]
    pub r006: Handle<Scene>,
    #[asset(path = "models/generated/R007.glb#Scene0")]
    pub r007: Handle<Scene>,
    #[asset(path = "models/generated/R008.glb#Scene0")]
    pub r008: Handle<Scene>,
    #[asset(path = "models/generated/R009.glb#Scene0")]
    pub r009: Handle<Scene>,
    #[asset(path = "models/generated/R010.glb#Scene0")]
    pub r010: Handle<Scene>,
}

//Is this safe? Can it be made more safe? I like the approach of bevy Reflect
impl Index<usize> for HexAssets {
    type Output = Handle<Scene>;

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
            27 => &self.c001,
            28 => &self.c002,
            29 => &self.c003,
            30 => &self.c004,
            31 => &self.c005,
            32 => &self.c006,
            33 => &self.c007,
            34 => &self.c008,

            35 => &self.dragon1,
            36 => &self.dragon2,
            37 => &self.dragon3,
            38 => &self.dragon4,
            39 => &self.dragon5,
            40 => &self.dragon6,
            41 => &self.dragon7,

            42 => &self.f001,
            43 => &self.f002,
            44 => &self.f003,
            45 => &self.f004,
            46 => &self.f005,
            47 => &self.f006,
            48 => &self.f007,
            49 => &self.f008,
            50 => &self.f009,
            51 => &self.f010,

            52 => &self.lra01,
            53 => &self.lra02,
            54 => &self.lra03,
            55 => &self.lra04,
            56 => &self.lra05,
            57 => &self.lra06,
            58 => &self.lra07,
            59 => &self.lra08,

            60 => &self.r001,
            61 => &self.r002,
            62 => &self.r003,
            63 => &self.r004,
            64 => &self.r005,
            65 => &self.r006,
            66 => &self.r007,
            67 => &self.r008,
            68 => &self.r009,
            69 => &self.r010,
            _ => panic!(),
        }
    }
}

impl HexAssets {
    pub fn get(&self, key: &str) -> &Handle<Scene> {
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
            _ => &self.blank,
        }
    }
}

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

            27 => &self.c001,
            28 => &self.c002,
            29 => &self.c003,
            30 => &self.c004,
            31 => &self.c005,
            32 => &self.c006,
            33 => &self.c007,
            34 => &self.c008,

            35 => &self.dragon1,
            36 => &self.dragon2,
            37 => &self.dragon3,
            38 => &self.dragon4,
            39 => &self.dragon5,
            40 => &self.dragon6,
            41 => &self.dragon7,

            42 => &self.f001,
            43 => &self.f002,
            44 => &self.f003,
            45 => &self.f004,
            46 => &self.f005,
            47 => &self.f006,
            48 => &self.f007,
            49 => &self.f008,
            50 => &self.f009,
            51 => &self.f010,

            52 => &self.lra01,
            53 => &self.lra02,
            54 => &self.lra03,
            55 => &self.lra04,
            56 => &self.lra05,
            57 => &self.lra06,
            58 => &self.lra07,
            59 => &self.lra08,

            60 => &self.r001,
            61 => &self.r002,
            62 => &self.r003,
            63 => &self.r004,
            64 => &self.r005,
            65 => &self.r006,
            66 => &self.r007,
            67 => &self.r008,
            68 => &self.r009,
            69 => &self.r010,
            _ => panic!(),
        }
    }
}
