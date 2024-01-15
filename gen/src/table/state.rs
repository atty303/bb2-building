use table::{EntityParser, TableParser};

pub struct StateTable;

#[derive(Debug)]
pub struct StateRow {
    row_id: String,
    name: String,
    id: String,
    order: usize,
    icon: String,
    category: String,
    format: String,
    long_format: String,
    text_color: String,
    value_type: String,
    calc_logic: String,
    in_param_list: bool,
    power: i32,
    base_value: i32,
    max_value: i32,
    rarity: i32,
    matrix: String,
    is_good: bool,
    need_calc: bool,
    tag: String,
    pop_random: bool,
    pop_type: i32,
    sstate: String,
    state_act: String,
    state_calc: String,
    state_visual: String,
    desc: String,
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
