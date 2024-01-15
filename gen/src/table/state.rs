use table::{EntityParser, TableParser};

pub struct StateTable;

#[derive(Debug)]
pub struct StateRow {
    pub row_id: String,
    pub name: String,
    pub id: String,
    pub order: usize,
    pub icon: String,
    pub category: String,
    pub format: String,
    pub long_format: String,
    pub text_color: String,
    pub value_type: String,
    pub calc_logic: String,
    pub in_param_list: bool,
    pub power: i32,
    pub base_value: i32,
    pub max_value: i32,
    pub rarity: i32,
    pub matrix: String,
    pub is_good: bool,
    pub need_calc: bool,
    pub tag: String,
    pub pop_random: bool,
    pub pop_type: i32,
    pub sstate: String,
    pub state_act: String,
    pub state_calc: String,
    pub state_visual: String,
    pub desc: String,
}

impl TableParser for StateTable {
    type Row = StateRow;
    fn parse_row(p: &EntityParser) -> Self::Row {
        StateRow {
            row_id: p.row_id(),
            name: p.get_str("name"),
            id: p.get_str("ID"),
            order: p.get_usize("Order"),
            icon: p.get_str("Icon"),
            category: p.get_str("Category"),
            format: p.get_str("Format"),
            long_format: p.get_str("LongFormat"),
            text_color: p.get_str("TextColor"),
            value_type: p.get_str("ValueType"),
            calc_logic: p.get_str("CalcLogic"),
            in_param_list: p.get_bool("InParamList"),
            power: p.get_i32("Power"),
            base_value: p.get_i32("BaseValue"),
            max_value: p.get_i32("MaxValue"),
            rarity: p.get_i32("Rarity"),
            matrix: p.get_str("Matrix"),
            is_good: p.get_bool("IsGood"),
            need_calc: p.get_bool("NeedCalc"),
            tag: p.get_str("Tag"),
            pop_random: p.get_bool("PopRandom"),
            pop_type: p.get_i32("PopType"),
            sstate: p.get_str("sstate"),
            state_act: p.get_str("state_act"),
            state_calc: p.get_str("state_calc"),
            state_visual: p.get_str("state_visual"),
            desc: p.get_str("Desc"),
        }
    }
}
