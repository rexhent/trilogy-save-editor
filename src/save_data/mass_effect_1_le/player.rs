use serde::{Deserialize, Serialize};

use crate::save_data::{
    shared::{
        appearance::HeadMorph,
        player::{Notoriety, Origin},
    },
    Dummy, ImguiString,
};

// TODO: Tous les u8
#[derive(Deserialize, Serialize, RawUi, Clone)]
pub struct Player {
    pub is_female: bool,
    localized_class_name: i32,
    _player_class: u8,
    pub level: i32,
    pub current_xp: f32,
    pub first_name: ImguiString,
    localized_last_name: i32,
    pub origin: Origin,
    pub notoriety: Notoriety,
    specialization_bonus_id: i32,
    _spectre_rank: u8,
    pub talent_points: i32,
    talent_pool_points: i32,
    mapped_talent: ImguiString,
    pub head_morph: Option<HeadMorph>,
    simple_talents: Vec<SimpleTalent>,
    pub complex_talents: Vec<ComplexTalent>,
    pub inventory: Inventory,
    pub credits: i32,
    pub medigel: i32,
    pub grenades: f32,
    pub omnigel: f32,
    pub face_code: ImguiString,
    _unknown4: Dummy<4>,
    auto_levelup_template_id: i32,
    health_per_level: f32,
    _unknown5: Dummy<9>,
    stamina: i32,
    focus: i32,
    precision: i32,
    coordination: i32,
    _unknown6: Dummy<14>,
    health_current: f32,
}

#[derive(Deserialize, Serialize, RawUi, Clone, Default)]
pub struct SimpleTalent {
    talent_id: i32,
    current_rank: i32,
}

#[derive(Deserialize, Serialize, RawUi, Clone, Default)]
pub struct ComplexTalent {
    talent_id: i32,
    pub current_rank: i32,
    max_rank: i32,
    level_offset: i32,
    levels_per_rank: i32,
    visual_order: i32,
    prereq_talent_ids: Vec<i32>,
    prereq_talent_ranks: Vec<i32>,
}

#[derive(Deserialize, Serialize, RawUi, Clone, Default)]
pub struct Inventory {
    pub equipped: Vec<Item>,
    pub quick_slots: Vec<Item>,
    pub inventory: Vec<Item>,
    plot_items: Vec<Item>,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Deserialize, Serialize, RawUi, Clone)]
pub enum ItemLevel {
    None,
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
    IX,
    X,
}

impl Default for ItemLevel {
    fn default() -> Self {
        ItemLevel::None
    }
}

#[derive(Deserialize, Serialize, RawUi, Clone, Default)]
pub struct Item {
    pub item_id: i32,
    pub item_level: ItemLevel,
    pub manufacturer_id: i32,
    plot_conditional_id: i32,
    new_item: bool,
    junk: bool,
    attached_mods: Vec<ItemMod>,
}

#[derive(Deserialize, Serialize, RawUi, Clone, Default)]
struct ItemMod {
    item_id: i32,
    item_level: ItemLevel,
    manufacturer_id: i32,
    plot_conditional_id: i32,
}
