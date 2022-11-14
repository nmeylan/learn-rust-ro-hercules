#[derive(sqlx::FromRow, Debug)]
pub struct ItemModel {
    #[sqlx(default)]
    pub id: Option<u32>,
    #[sqlx(default)]
    pub name_aegis: Option<String>,
    #[sqlx(default)]
    pub name_english: Option<String>,
    #[sqlx(rename = "type")]
    #[sqlx(default)]
    pub item_type: Option<String>,
    #[sqlx(default)]
    pub subtype: Option<String>,
    #[sqlx(default)]
    pub price_buy: Option<u32>,
    #[sqlx(default)]
    pub price_sell: Option<u32>,
    #[sqlx(default)]
    pub weight: Option<u16>,
    #[sqlx(default)]
    pub attack: Option<u16>,
    #[sqlx(default)]
    pub defense: Option<u16>,
    #[sqlx(default)]
    pub range: Option<u8>,
    #[sqlx(default)]
    pub slots: Option<u8>,
    #[sqlx(default)]
    pub job_all: Option<bool>,
    #[sqlx(default)]
    pub job_acolyte: Option<bool>,
    #[sqlx(default)]
    pub job_alchemist: Option<bool>,
    #[sqlx(default)]
    pub job_archer: Option<bool>,
    #[sqlx(default)]
    pub job_assassin: Option<bool>,
    #[sqlx(default)]
    pub job_barddancer: Option<bool>,
    #[sqlx(default)]
    pub job_blacksmith: Option<bool>,
    #[sqlx(default)]
    pub job_crusader: Option<bool>,
    #[sqlx(default)]
    pub job_gunslinger: Option<bool>,
    #[sqlx(default)]
    pub job_hunter: Option<bool>,
    #[sqlx(default)]
    pub job_knight: Option<bool>,
    #[sqlx(default)]
    pub job_mage: Option<bool>,
    #[sqlx(default)]
    pub job_merchant: Option<bool>,
    #[sqlx(default)]
    pub job_monk: Option<bool>,
    #[sqlx(default)]
    pub job_ninja: Option<bool>,
    #[sqlx(default)]
    pub job_novice: Option<bool>,
    #[sqlx(default)]
    pub job_priest: Option<bool>,
    #[sqlx(default)]
    pub job_rogue: Option<bool>,
    #[sqlx(default)]
    pub job_sage: Option<bool>,
    #[sqlx(default)]
    pub job_soullinker: Option<bool>,
    #[sqlx(default)]
    pub job_stargladiator: Option<bool>,
    #[sqlx(default)]
    pub job_supernovice: Option<bool>,
    #[sqlx(default)]
    pub job_swordman: Option<bool>,
    #[sqlx(default)]
    pub job_taekwon: Option<bool>,
    #[sqlx(default)]
    pub job_thief: Option<bool>,
    #[sqlx(default)]
    pub job_wizard: Option<bool>,
    #[sqlx(default)]
    pub class_all: Option<bool>,
    #[sqlx(default)]
    pub class_normal: Option<bool>,
    #[sqlx(default)]
    pub class_upper: Option<bool>,
    #[sqlx(default)]
    pub class_baby: Option<bool>,
    #[sqlx(default)]
    pub gender: Option<String>,
    #[sqlx(default)]
    pub location_head_top: Option<bool>,
    #[sqlx(default)]
    pub location_head_mid: Option<bool>,
    #[sqlx(default)]
    pub location_head_low: Option<bool>,
    #[sqlx(default)]
    pub location_armor: Option<bool>,
    #[sqlx(default)]
    pub location_right_hand: Option<bool>,
    #[sqlx(default)]
    pub location_left_hand: Option<bool>,
    #[sqlx(default)]
    pub location_garment: Option<bool>,
    #[sqlx(default)]
    pub location_shoes: Option<bool>,
    #[sqlx(default)]
    pub location_right_accessory: Option<bool>,
    #[sqlx(default)]
    pub location_left_accessory: Option<bool>,
    #[sqlx(default)]
    pub location_costume_head_top: Option<bool>,
    #[sqlx(default)]
    pub location_costume_head_mid: Option<bool>,
    #[sqlx(default)]
    pub location_costume_head_low: Option<bool>,
    #[sqlx(default)]
    pub location_costume_garment: Option<bool>,
    #[sqlx(default)]
    pub location_ammo: Option<bool>,
    #[sqlx(default)]
    pub location_shadow_armor: Option<bool>,
    #[sqlx(default)]
    pub location_shadow_weapon: Option<bool>,
    #[sqlx(default)]
    pub location_shadow_shield: Option<bool>,
    #[sqlx(default)]
    pub location_shadow_shoes: Option<bool>,
    #[sqlx(default)]
    pub location_shadow_right_accessory: Option<bool>,
    #[sqlx(default)]
    pub location_shadow_left_accessory: Option<bool>,
    #[sqlx(default)]
    pub weapon_level: Option<u8>,
    #[sqlx(default)]
    pub armor_level: Option<u8>,
    #[sqlx(default)]
    pub equip_level_min: Option<u8>,
    #[sqlx(default)]
    pub equip_level_max: Option<u8>,
    #[sqlx(default)]
    pub refineable: Option<u8>,
    #[sqlx(default)]
    pub view: Option<u16>,
    #[sqlx(default)]
    pub alias_name: Option<String>,
    #[sqlx(default)]
    pub flag_buyingstore: Option<bool>,
    #[sqlx(default)]
    pub flag_deadbranch: Option<bool>,
    #[sqlx(default)]
    pub flag_container: Option<bool>,
    #[sqlx(default)]
    pub flag_uniqueid: Option<bool>,
    #[sqlx(default)]
    pub flag_bindonequip: Option<bool>,
    #[sqlx(default)]
    pub flag_dropannounce: Option<bool>,
    #[sqlx(default)]
    pub flag_noconsume: Option<bool>,
    #[sqlx(default)]
    pub flag_dropeffect: Option<String>,
    #[sqlx(default)]
    pub delay_duration: Option<u64>,
    #[sqlx(default)]
    pub delay_status: Option<String>,
    #[sqlx(default)]
    pub stack_amount: Option<u16>,
    #[sqlx(default)]
    pub stack_inventory: Option<bool>,
    #[sqlx(default)]
    pub stack_cart: Option<bool>,
    #[sqlx(default)]
    pub stack_storage: Option<bool>,
    #[sqlx(default)]
    pub stack_guildstorage: Option<bool>,
    #[sqlx(default)]
    pub nouse_override: Option<u16>,
    #[sqlx(default)]
    pub nouse_sitting: Option<bool>,
    #[sqlx(default)]
    pub trade_override: Option<u16>,
    #[sqlx(default)]
    pub trade_nodrop: Option<bool>,
    #[sqlx(default)]
    pub trade_notrade: Option<bool>,
    #[sqlx(default)]
    pub trade_tradepartner: Option<bool>,
    #[sqlx(default)]
    pub trade_nosell: Option<bool>,
    #[sqlx(default)]
    pub trade_nocart: Option<bool>,
    #[sqlx(default)]
    pub trade_nostorage: Option<bool>,
    #[sqlx(default)]
    pub trade_noguildstorage: Option<bool>,
    #[sqlx(default)]
    pub trade_nomail: Option<bool>,
    #[sqlx(default)]
    pub trade_noauction: Option<bool>,
    #[sqlx(default)]
    pub script: Option<String>,
    #[sqlx(default)]
    pub equip_script: Option<String>,
    #[sqlx(default)]
    pub unequip_script: Option<String>,
}
#[derive(sqlx::FromRow, Debug)]
pub struct ItemBuySellModel {
    pub id: Option<u32>,
    #[sqlx(rename = "type")]
    pub item_type: String,
    #[sqlx(default)]
    pub price_buy: Option<u32>,
    #[sqlx(default)]
    pub price_sell: Option<u32>,
}