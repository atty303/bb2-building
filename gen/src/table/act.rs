use table::{EntityParser, TableParser};

pub struct ActTable;

#[derive(Debug)]
pub struct ActRow {
    pub row_id: String,
    pub id: String,
    pub name: String,
    pub order: usize,
    pub act_node: String,
    pub tag: String,
    pub link_key: String,
    pub is_rune: bool,
    pub namer: String,
}

impl TableParser for ActTable {
    type Row = ActRow;
    fn parse_row(p: &EntityParser) -> Self::Row {
        ActRow {
            row_id: p.row_id(),
            id: p.get_str("ID"),
            name: p.get_str("name"),
            order: p.get_usize("Order"),
            act_node: p.get_str("act_node"),
            tag: p.get_str("Tag"),
            link_key: p.get_str("LinkKey"),
            is_rune: p.get_bool("IsRune"),
            namer: p.get_str("namer"),
        }
    }
}
