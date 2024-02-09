use table::{EntityParser, TableParser};

pub struct SkillTable;

#[derive(Debug)]
pub struct SkillRow {
    pub row_id: String,
    pub name: String,
    pub id: String,
    pub order: usize,
    pub icon: String,
    pub category: String,
    pub poss_num: i32,
    pub for_user: bool,
    pub on_dict: bool,
    pub rarity: i32,
    pub freq: i32,
    pub skill_mode: String,
    pub aff1: i32,
    pub aff2: i32,
    pub aff3: i32,
    pub aff4: i32,
    pub audio: String,
    pub in_dict: bool,
    pub drop: bool,
    pub tag: String,
    pub is_free: bool,
    pub seed: i32,
    pub enable: String,
}

impl TableParser for SkillTable {
    type Row = SkillRow;
    fn parse_row(p: &EntityParser) -> Self::Row {
        SkillRow {
            row_id: p.row_id(),
            name: p.get_str("name"),
            id: p.get_str("ID"),
            order: p.get_usize("Order"),
            icon: p.get_str("Icon"),
            category: p.get_str("Category"),
            poss_num: p.get_i32("PossNum"),
            for_user: p.get_bool("ForUser"),
            on_dict: p.get_bool("OnDict"),
            rarity: p.get_i32("Rarity"),
            freq: p.get_i32("Freq"),
            skill_mode: p.get_str("skill_mode"),
            aff1: p.get_i32("Aff1"),
            aff2: p.get_i32("Aff2"),
            aff3: p.get_i32("Aff3"),
            aff4: p.get_i32("Aff4"),
            audio: p.get_str("audio"),
            in_dict: p.get_bool("InDict"),
            drop: p.get_bool("Drop"),
            tag: p.get_str("Tag"),
            is_free: p.get_bool("IsFree"),
            seed: p.get_i32("Seed"),
            enable: p.get_str("Enable"),
        }
    }
}
