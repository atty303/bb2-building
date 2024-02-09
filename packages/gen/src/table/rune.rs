use table::{EntityParser, TableParser};

pub struct RuneTable;

#[derive(Debug)]
pub struct RuneRow {
    pub row_id: String,
    pub name: String,
    pub id: String,
    pub order: usize,
    pub icon: String,
    pub rarity: i32,
    pub freq: i32,
    pub cap_cost: i32,
    pub audio: String,
    pub param_key: String,
    pub power: i32,
    pub in_dict: bool,
    pub tag: String,
    pub rune_act: String,
    pub desc: String,
    pub short_text: String,
    pub short_color: String,
    pub for_random: bool,
    pub enable: String,
}

impl TableParser for RuneTable {
    type Row = RuneRow;
    fn parse_row(p: &EntityParser) -> Self::Row {
        RuneRow {
            row_id: p.row_id(),
            name: p.get_str("name"),
            id: p.get_str("ID"),
            order: p.get_usize("Order"),
            icon: p.get_str("Icon"),
            rarity: p.get_i32("Rarity"),
            freq: p.get_i32("Freq"),
            cap_cost: p.get_i32("CapCost"),
            audio: p.get_str("audio"),
            param_key: p.get_str("ParamKey"),
            power: p.get_i32("Power"),
            in_dict: p.get_bool("InDict"),
            tag: p.get_str("Tag"),
            rune_act: p.get_str("rune_act"),
            desc: p.get_str("Desc"),
            short_text: p.get_str("ShortText"),
            short_color: p.get_str("ShortColor"),
            for_random: p.get_bool("ForRandom"),
            enable: p.get_str("Enable"),
        }
    }
}
