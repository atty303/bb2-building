use table::{EntityParser, TableParser};

pub struct SkillModeTable;

#[derive(Debug)]
pub struct SkillModeRow {
    pub row_id: String,
    pub id: String,
    pub name: String,
    /// skill_mode(N) = skill(1) relation (format: `skill_mode.{}_{}_{}`)
    pub skill: String,
    pub order: usize,
    pub icon: String,
    pub category: String,
    pub alt_mode: bool,
    pub is_brave: bool,
    pub use_num: i32,
    pub use_brave: i32,
    pub cooldown: i32,
    pub use_init: bool,
    pub is_quick: bool,
    pub sm_act: String,
    pub skill_tag: String,
}

impl TableParser for SkillModeTable {
    type Row = SkillModeRow;
    fn parse_row(p: &EntityParser) -> Self::Row {
        SkillModeRow {
            row_id: p.row_id(),
            id: p.get_str("ID"),
            name: p.get_str("name"),
            skill: p.get_str("skill"),
            order: p.get_usize("Order"),
            icon: p.get_str("Icon"),
            category: p.get_str("Category"),
            alt_mode: p.get_bool("AltMode"),
            is_brave: p.get_bool("IsBrave"),
            use_num: p.get_i32("UseNum"),
            use_brave: p.get_i32("UseBrave"),
            cooldown: p.get_i32("Cooldown"),
            use_init: p.get_bool("UseInit"),
            is_quick: p.get_bool("IsQuick"),
            sm_act: p.get_str("sm_act"),
            skill_tag: p.get_str("SkillTag"),
        }
    }
}
