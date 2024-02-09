use table::{EntityParser, TableParser};

pub struct SmActTable;

#[derive(Debug)]
pub struct SmActRow {
    pub row_id: String,
    pub id: String,
    pub name: String,
    pub skill_mode: String,
    pub act: String,
    pub act_trigger: String,
    pub freq: i32,
}

impl TableParser for SmActTable {
    type Row = SmActRow;
    fn parse_row(p: &EntityParser) -> Self::Row {
        SmActRow {
            row_id: p.row_id(),
            id: p.get_str("ID"),
            name: p.get_str("name"),
            skill_mode: p.get_str("skill_mode"),
            act: p.get_str("act"),
            act_trigger: p.get_str("ActTrigger"),
            freq: p.get_i32("Freq"),
        }
    }
}
