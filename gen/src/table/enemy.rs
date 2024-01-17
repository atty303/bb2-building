use table::{EntityParser, TableParser};

pub struct EnemyTable;

#[derive(Debug)]
pub struct EnemyRow {
    pub row_id: String,
    pub name: String,
    pub id: String,
    pub order: usize,
    pub image: String,
    pub icon: String,
    pub rarity: i32,
    pub max_hp: i32,
    pub max_shield: i32,
    pub hp_stack: i32,
    pub str: i32,
    pub dex: i32,
    pub int: i32,
    pub pie: i32,
    pub enemy_type: String,
    pub enemy_skill: String,
    pub bgm: String,
    pub enemy_drop: String,
    pub in_dict: bool,
    pub category: String,
    pub drop_ss: i32,
    pub drop_exp: i32,
    pub tag: String,
    pub camp_item: String,
    pub seed: i32,
    pub enable: String,
}

impl TableParser for EnemyTable {
    type Row = EnemyRow;
    fn parse_row(p: &EntityParser) -> Self::Row {
        EnemyRow {
            row_id: p.row_id(),
            name: p.get_str("name"),
            id: p.get_str("ID"),
            order: p.get_usize("Order"),
            image: p.get_str("Image"),
            icon: p.get_str("Icon"),
            rarity: p.get_i32("Rarity"),
            max_hp: p.get_i32("MaxHP"),
            max_shield: p.get_i32("MaxShield"),
            hp_stack: p.get_i32("HPStack"),
            str: p.get_i32("STR"),
            dex: p.get_i32("DEX"),
            int: p.get_i32("INT"),
            pie: p.get_i32("PIE"),
            enemy_type: p.get_str("enemy_type"),
            enemy_skill: p.get_str("enemy_skill"),
            bgm: p.get_str("BGM"),
            enemy_drop: p.get_str("enemy_drop"),
            in_dict: p.get_bool("InDict"),
            category: p.get_str("Category"),
            drop_ss: p.get_i32("DropSS"),
            drop_exp: p.get_i32("DropExp"),
            tag: p.get_str("Tag"),
            camp_item: p.get_str("CampItem"),
            seed: p.get_i32("Seed"),
            enable: p.get_str("Enable"),
        }
    }
}
