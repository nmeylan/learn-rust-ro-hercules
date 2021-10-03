#![allow(dead_code)]

pub enum StatusTypes {
    NONE,
    SPEED,
    BASEEXP,
    JOBEXP,
    KARMA,
    MANNER,
    HP,
    MAXHP,
    SP,
    MAXSP,
    STATUSPOINT,
    BASELEVEL,
    SKILLPOINT,
    STR,
    AGI,
    VIT,
    INT,
    DEX,
    LUK,
    CLASS,
    ZENY,
    SEX,
    NEXTBASEEXP,
    NEXTJOBEXP,
    WEIGHT,
    MAXWEIGHT,
    USTR,
    UAGI,
    UVIT,
    UINT,
    UDEX,
    ULUK,
    ATK1,
    ATK2,
    MATK1,
    MATK2,
    DEF1,
    DEF2,
    MDEF1,
    MDEF2,
    HIT,
    FLEE1,
    FLEE2,
    CRITICAL,
    ASPD,
    JOBLEVEL,
    UPPER,
    PARTNER,
    CART,
    FAME,
    UNBREAKABLE,
    CARTINFO,
    BASEJOB,
    BASECLASS,
    KILLERRID,
    KILLEDRID,
    SLOTCHANGE,
    CHARRENAME,
    MODEXP,
    MODDROP,
    MODDEATH,
    BANKVAULT,
    MERCFLEE,
    MERCKILLS,
    MERCFAITH,
    POW,
    STA,
    WIS,
    SPL,
    CON,
    CRT,
    PATK,
    SMATK,
    RES,
    MRES,
    HPLUS,
    CRATE,
    TSTATUSPOINT,
    AP,
    MAXAP,
    UPOW,
    USTA,
    UWIS,
    USPL,
    UCON,
    UCRT,
    ATTACKRANGE,
    ATKELE,
    DEFELE,
    CASTRATE,
    MAXHPRATE,
    MAXSPRATE,
    SPRATE,
    ADDELE,
    ADDRACE,
    ADDSIZE,
    SUBELE,
    SUBRACE,
    ADDEFF,
    RESEFF,
    BASEATK,
    ASPDRATE,
    HPRECOVRATE,
    RECOVRATE,
    SPEEDRATE,
    CRITICALDEF,
    NEARATKDEF,
    LONGATKDEF,
    DOUBLERATE,
    DOUBLEADDRATE,
    SKILLHEAL,
    MATKRATE,
    IGNOREDEFELE,
    IGNOREDEFRACE,
    ATKRATE,
    SPEEDADDRATE,
    REGENRATE,
    MAGICATKDEF,
    MISCATKDEF,
    IGNOREMDEFELE,
    IGNOREMDEFRACE,
    MAGICADDELE,
    MAGICADDRACE,
    MAGICADDSIZE,
    PERFECTHITRATE,
    PERFECTHITADDRATE,
    CRITICALRATE,
    GETZENYNUM,
    ADDGETZENYNUM,
    ADDDAMAGECLASS,
    ADDMAGICDAMAGECLASS,
    ADDDEFCLASS,
    ADDMDEFCLASS,
    ADDMONSTERDROPITEM,
    DEFRATIOATKELE,
    DEFRATIOATKRACE,
    UNBREAKABLEGARMENT,
    HITRATE,
    FLEERATE,
    FLEE2RATE,
    DEFRATE,
    DEF2RATE,
    MDEFRATE,
    MDEF2RATE,
    SPLASHRANGE,
    SPLASHADDRANGE,
    AUTOSPELL,
    HPDRAINRATE,
    DRAINRATE,
    SHORTWEAPONDAMAGERETURN,
    LONGWEAPONDAMAGERETURN,
    WEAPONCOMAELE,
    WEAPONCOMARACE,
    ADDEFF2,
    BREAKWEAPONRATE,
    BREAKARMORRATE,
    ADDSTEALRATE,
    MAGICDAMAGERETURN,
    ALLSTATS,
    AGIVIT,
    AGIDEXSTR,
    PERFECTHIDE,
    NOKNOCKBACK,
    CLASSCHANGE,
    HPDRAINVALUE,
    DRAINVALUE,
    WEAPONATK,
    WEAPONATKRATE,
    DELAYRATE,
    HPDRAINRATERACE,
    DRAINRATERACE,
    IGNOREMDEFRATE,
    IGNOREDEFRATE,
    SKILLHEAL2,
    ADDEFFONSKILL,
    ADDHEALRATE,
    ADDHEAL2RATE,
    HPVANISHRATE,
    RESTARTFULLRECOVER,
    NOCASTCANCEL,
    NOSIZEFIX,
    NOMAGICDAMAGE,
    NOWEAPONDAMAGE,
    NOGEMSTONE,
    NOCASTCANCEL2,
    NOMISCDAMAGE,
    UNBREAKABLEWEAPON,
    UNBREAKABLEARMOR,
    UNBREAKABLEHELM,
    UNBREAKABLESHIELD,
    LONGATKRATE,
    CRITATKRATE,
    CRITICALADDRACE,
    NOREGEN,
    ADDEFFWHENHIT,
    AUTOSPELLWHENHIT,
    SKILLATK,
    UNSTRIPABLE,
    AUTOSPELLONSKILL,
    GAINVALUE,
    HPREGENRATE,
    HPLOSSRATE,
    ADDRACE2,
    HPGAINVALUE,
    SUBSIZE,
    HPDRAINVALUERACE,
    ADDITEMHEALRATE,
    DRAINVALUERACE,
    EXPADDRACE,
    GAINRACE,
    SUBRACE2,
    UNBREAKABLESHOES,
    UNSTRIPABLEWEAPON,
    UNSTRIPABLEARMOR,
    UNSTRIPABLEHELM,
    UNSTRIPABLESHIELD,
    INTRAVISION,
    ADDMONSTERDROPCHAINITEM,
    LOSSRATE,
    ADDSKILLBLOW,
    VANISHRATE,
    MAGICGAINVALUE,
    MAGICHPGAINVALUE,
    ADDCLASSDROPITEM,
    EMATK,
    GAINRACEATTACK,
    HPGAINRACEATTACK,
    SKILLUSERATE,
    SKILLCOOLDOWN,
    SKILLFIXEDCAST,
    SKILLVARIABLECAST,
    FIXCASTRATE,
    VARCASTRATE,
    SKILLUSESP,
    MAGICATKELE,
    ADDFIXEDCAST,
    ADDVARIABLECAST,
    SETDEFRACE,
    SETMDEFRACE,
    RACETOLERANCE,
    ADDMAXWEIGHT,
    SUBDEFELE,
    MAGICSUBDEFELE,
    STATENORECOVERRACE,
}

impl StatusTypes {
    pub fn value(&self) -> u32 {
        match *self {
            StatusTypes::NONE => u32::MAX,
            StatusTypes::SPEED => 0,
            StatusTypes::BASEEXP => 1,
            StatusTypes::JOBEXP => 2,
            StatusTypes::KARMA => 3,
            StatusTypes::MANNER => 4,
            StatusTypes::HP => 5,
            StatusTypes::MAXHP => 6,
            StatusTypes::SP => 7,
            StatusTypes::MAXSP => 8,
            StatusTypes::STATUSPOINT => 9,
            StatusTypes::BASELEVEL => 11,
            StatusTypes::SKILLPOINT => 12,
            StatusTypes::STR => 13,
            StatusTypes::AGI => 14,
            StatusTypes::VIT => 15,
            StatusTypes::INT => 16,
            StatusTypes::DEX => 17,
            StatusTypes::LUK => 18,
            StatusTypes::CLASS => 19,
            StatusTypes::ZENY => 20,
            StatusTypes::SEX => 21,
            StatusTypes::NEXTBASEEXP => 22,
            StatusTypes::NEXTJOBEXP => 23,
            StatusTypes::WEIGHT => 24,
            StatusTypes::MAXWEIGHT => 25,
            StatusTypes::USTR => 32,
            StatusTypes::UAGI => 33,
            StatusTypes::UVIT => 34,
            StatusTypes::UINT => 35,
            StatusTypes::UDEX => 36,
            StatusTypes::ULUK => 37,
            StatusTypes::ATK1 => 41,
            StatusTypes::ATK2 => 42,
            StatusTypes::MATK1 => 43,
            StatusTypes::MATK2 => 44,
            StatusTypes::DEF1 => 45,
            StatusTypes::DEF2 => 46,
            StatusTypes::MDEF1 => 47,
            StatusTypes::MDEF2 => 48,
            StatusTypes::HIT => 49,
            StatusTypes::FLEE1 => 50,
            StatusTypes::FLEE2 => 51,
            StatusTypes::CRITICAL => 52,
            StatusTypes::ASPD => 53,
            StatusTypes::JOBLEVEL => 55,
            StatusTypes::UPPER => 56,
            StatusTypes::PARTNER => 57,
            StatusTypes::CART => 58,
            StatusTypes::FAME => 59,
            StatusTypes::UNBREAKABLE => 60,
            StatusTypes::CARTINFO => 99,
            StatusTypes::BASEJOB => 119,
            StatusTypes::BASECLASS => 120,
            StatusTypes::KILLERRID => 121,
            StatusTypes::KILLEDRID => 122,
            StatusTypes::SLOTCHANGE => 123,
            StatusTypes::CHARRENAME => 124,
            StatusTypes::MODEXP => 125,
            StatusTypes::MODDROP => 126,
            StatusTypes::MODDEATH => 127,
            StatusTypes::BANKVAULT => 128,

            // Mercenaries
            StatusTypes::MERCFLEE => 165,
            StatusTypes::MERCKILLS => 189,
            StatusTypes::MERCFAITH => 190,

            // 4th job update
            StatusTypes::POW => 219,
            StatusTypes::STA => 220,
            StatusTypes::WIS => 221,
            StatusTypes::SPL => 222,
            StatusTypes::CON => 223,
            StatusTypes::CRT => 224,
            StatusTypes::PATK => 225,
            StatusTypes::SMATK => 226,
            StatusTypes::RES => 227,
            StatusTypes::MRES => 228,
            StatusTypes::HPLUS => 229,
            StatusTypes::CRATE => 230,
            StatusTypes::TSTATUSPOINT => 231,
            StatusTypes::AP => 232,
            StatusTypes::MAXAP => 233,
            StatusTypes::UPOW => 247,
            StatusTypes::USTA => 248,
            StatusTypes::UWIS => 249,
            StatusTypes::USPL => 250,
            StatusTypes::UCON => 251,
            StatusTypes::UCRT => 252,

            // original 1000-
            StatusTypes::ATTACKRANGE => 1000,
            StatusTypes::ATKELE => 1001,
            StatusTypes::DEFELE => 1002,
            StatusTypes::CASTRATE => 1003,
            StatusTypes::MAXHPRATE => 1004,
            StatusTypes::MAXSPRATE => 1005,
            StatusTypes::SPRATE => 1006,
            StatusTypes::ADDELE => 1007,
            StatusTypes::ADDRACE => 1008,
            StatusTypes::ADDSIZE => 1009,
            StatusTypes::SUBELE => 1010,
            StatusTypes::SUBRACE => 1011,
            StatusTypes::ADDEFF => 1012,
            StatusTypes::RESEFF => 1013,
            StatusTypes::BASEATK => 1014,
            StatusTypes::ASPDRATE => 1015,
            StatusTypes::HPRECOVRATE => 1016,
            StatusTypes::RECOVRATE => 1017,
            StatusTypes::SPEEDRATE => 1018,
            StatusTypes::CRITICALDEF => 1019,
            StatusTypes::NEARATKDEF => 1020,
            StatusTypes::LONGATKDEF => 1021,
            StatusTypes::DOUBLERATE => 1022,
            StatusTypes::DOUBLEADDRATE => 1023,
            StatusTypes::SKILLHEAL => 1024,
            StatusTypes::MATKRATE => 1025,
            StatusTypes::IGNOREDEFELE => 1026,
            StatusTypes::IGNOREDEFRACE => 1027,
            StatusTypes::ATKRATE => 1028,
            StatusTypes::SPEEDADDRATE => 1029,
            StatusTypes::REGENRATE => 1030,
            StatusTypes::MAGICATKDEF => 1031,
            StatusTypes::MISCATKDEF => 1032,
            StatusTypes::IGNOREMDEFELE => 1033,
            StatusTypes::IGNOREMDEFRACE => 1034,
            StatusTypes::MAGICADDELE => 1035,
            StatusTypes::MAGICADDRACE => 1036,
            StatusTypes::MAGICADDSIZE => 1037,
            StatusTypes::PERFECTHITRATE => 1038,
            StatusTypes::PERFECTHITADDRATE => 1039,
            StatusTypes::CRITICALRATE => 1040,
            StatusTypes::GETZENYNUM => 1041,
            StatusTypes::ADDGETZENYNUM => 1042,
            StatusTypes::ADDDAMAGECLASS => 1043,
            StatusTypes::ADDMAGICDAMAGECLASS => 1044,
            StatusTypes::ADDDEFCLASS => 1045,
            StatusTypes::ADDMDEFCLASS => 1046,
            StatusTypes::ADDMONSTERDROPITEM => 1047,
            StatusTypes::DEFRATIOATKELE => 1048,
            StatusTypes::DEFRATIOATKRACE => 1049,
            StatusTypes::UNBREAKABLEGARMENT => 1050,
            StatusTypes::HITRATE => 1051,
            StatusTypes::FLEERATE => 1052,
            StatusTypes::FLEE2RATE => 1053,
            StatusTypes::DEFRATE => 1054,
            StatusTypes::DEF2RATE => 1055,
            StatusTypes::MDEFRATE => 1056,
            StatusTypes::MDEF2RATE => 1057,
            StatusTypes::SPLASHRANGE => 1058,
            StatusTypes::SPLASHADDRANGE => 1059,
            StatusTypes::AUTOSPELL => 1060,
            StatusTypes::HPDRAINRATE => 1061,
            StatusTypes::DRAINRATE => 1062,
            StatusTypes::SHORTWEAPONDAMAGERETURN => 1063,
            StatusTypes::LONGWEAPONDAMAGERETURN => 1064,
            StatusTypes::WEAPONCOMAELE => 1065,
            StatusTypes::WEAPONCOMARACE => 1066,
            StatusTypes::ADDEFF2 => 1067,
            StatusTypes::BREAKWEAPONRATE => 1068,
            StatusTypes::BREAKARMORRATE => 1069,
            StatusTypes::ADDSTEALRATE => 1070,
            StatusTypes::MAGICDAMAGERETURN => 1071,
            StatusTypes::ALLSTATS => 1073,
            StatusTypes::AGIVIT => 1074,
            StatusTypes::AGIDEXSTR => 1075,
            StatusTypes::PERFECTHIDE => 1076,
            StatusTypes::NOKNOCKBACK => 1077,
            StatusTypes::CLASSCHANGE => 1078,
            StatusTypes::HPDRAINVALUE => 1079,
            StatusTypes::DRAINVALUE => 1080,
            StatusTypes::WEAPONATK => 1081,
            StatusTypes::WEAPONATKRATE => 1082,
            StatusTypes::DELAYRATE => 1083,
            StatusTypes::HPDRAINRATERACE => 1084,
            StatusTypes::DRAINRATERACE => 1085,
            StatusTypes::IGNOREMDEFRATE => 1086,
            StatusTypes::IGNOREDEFRATE => 1087,
            StatusTypes::SKILLHEAL2 => 1088,
            StatusTypes::ADDEFFONSKILL => 1089,
            StatusTypes::ADDHEALRATE => 1090,
            StatusTypes::ADDHEAL2RATE => 1091,
            StatusTypes::HPVANISHRATE => 1092,
            StatusTypes::RESTARTFULLRECOVER => 2000,
            StatusTypes::NOCASTCANCEL => 2001,
            StatusTypes::NOSIZEFIX => 2002,
            StatusTypes::NOMAGICDAMAGE => 2003,
            StatusTypes::NOWEAPONDAMAGE => 2004,
            StatusTypes::NOGEMSTONE => 2005,
            StatusTypes::NOCASTCANCEL2 => 2006,
            StatusTypes::NOMISCDAMAGE => 2007,
            StatusTypes::UNBREAKABLEWEAPON => 2008,
            StatusTypes::UNBREAKABLEARMOR => 2009,
            StatusTypes::UNBREAKABLEHELM => 2010,
            StatusTypes::UNBREAKABLESHIELD => 2011,
            StatusTypes::LONGATKRATE => 2012,
            StatusTypes::CRITATKRATE => 2013,
            StatusTypes::CRITICALADDRACE => 2014,
            StatusTypes::NOREGEN => 2015,
            StatusTypes::ADDEFFWHENHIT => 2016,
            StatusTypes::AUTOSPELLWHENHIT => 2017,
            StatusTypes::SKILLATK => 2018,
            StatusTypes::UNSTRIPABLE => 2019,
            StatusTypes::AUTOSPELLONSKILL => 2020,
            StatusTypes::GAINVALUE => 2021,
            StatusTypes::HPREGENRATE => 2022,
            StatusTypes::HPLOSSRATE => 2023,
            StatusTypes::ADDRACE2 => 2024,
            StatusTypes::HPGAINVALUE => 2025,
            StatusTypes::SUBSIZE => 2026,
            StatusTypes::HPDRAINVALUERACE => 2027,
            StatusTypes::ADDITEMHEALRATE => 2028,
            StatusTypes::DRAINVALUERACE => 2029,
            StatusTypes::EXPADDRACE => 2030,
            StatusTypes::GAINRACE => 2031,
            StatusTypes::SUBRACE2 => 2032,
            StatusTypes::UNBREAKABLESHOES => 2033,
            StatusTypes::UNSTRIPABLEWEAPON => 2034,
            StatusTypes::UNSTRIPABLEARMOR => 2035,
            StatusTypes::UNSTRIPABLEHELM => 2036,
            StatusTypes::UNSTRIPABLESHIELD => 2037,
            StatusTypes::INTRAVISION => 2038,
            StatusTypes::ADDMONSTERDROPCHAINITEM => 2039,
            StatusTypes::LOSSRATE => 2040,
            StatusTypes::ADDSKILLBLOW => 2041,
            StatusTypes::VANISHRATE => 2042,
            StatusTypes::MAGICGAINVALUE => 2043,
            StatusTypes::MAGICHPGAINVALUE => 2044,
            StatusTypes::ADDCLASSDROPITEM => 2045,
            StatusTypes::EMATK => 2046,
            StatusTypes::GAINRACEATTACK => 2047,
            StatusTypes::HPGAINRACEATTACK => 2048,
            StatusTypes::SKILLUSERATE => 2049,
            StatusTypes::SKILLCOOLDOWN => 2050,
            StatusTypes::SKILLFIXEDCAST => 2051,
            StatusTypes::SKILLVARIABLECAST => 2052,
            StatusTypes::FIXCASTRATE => 2053,
            StatusTypes::VARCASTRATE => 2054,
            StatusTypes::SKILLUSESP => 2055,
            StatusTypes::MAGICATKELE => 2056,
            StatusTypes::ADDFIXEDCAST => 2057,
            StatusTypes::ADDVARIABLECAST => 2058,
            StatusTypes::SETDEFRACE => 2059,
            StatusTypes::SETMDEFRACE => 2060,
            StatusTypes::RACETOLERANCE => 2061,
            StatusTypes::ADDMAXWEIGHT => 2062,
            StatusTypes::SUBDEFELE => 2063,
            StatusTypes::MAGICSUBDEFELE => 2064,
            StatusTypes::STATENORECOVERRACE => 2065,
        }
    }
}