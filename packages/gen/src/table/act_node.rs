use table::{EntityParser, RowRef, TableParser};

pub struct ActNodeTable;

#[derive(Debug)]
pub struct ActNodeRow {
    pub row_id: String,
    pub id: String,
    pub name: String,
    pub order: usize,
    pub act: String,
    pub p_order: usize,
    pub action_type: String,
    pub target: i32,
    pub param_key: String,
    pub any: String,
    pub hit_rate: i32,
    pub avoid_type: String,
    pub relate_target: String,
    pub relate: String,
    pub power: i32,
    pub reduce: String,
    pub can_crit: bool,
    pub speed: i32,
    pub delay: i32,
    pub skill_tag: String,
    pub cond: String,
    pub free_val: String,
    pub tag: String,
    pub freq: i32,
    pub inc_target: String,
    pub inc_relate: String,
    pub inc_power: i32,
    pub state_last: Vec<i32>,
    pub act_num: i32,
    pub crit_rate: i32,
    pub is_skill: bool,
    pub check_target: bool,
    // extra
    pub any_ref: Option<RowRef>,
}

impl TableParser for ActNodeTable {
    type Row = ActNodeRow;
    fn parse_row(p: &EntityParser) -> Self::Row {
        let state_last = p.get_str("StateLast");
        let last = state_last
            .split('|')
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(last.len(), 5, "invalid state_last: {}", state_last);

        let any_ref = RowRef::parse(&p.get_str("any"));

        ActNodeRow {
            row_id: p.row_id(),
            id: p.get_str("ID"),
            name: p.get_str("name"),
            order: p.get_usize("Order"),
            act: p.get_str("act"),
            p_order: p.get_usize("POrder"),
            action_type: p.get_str("ActionType"),
            target: p.get_i32("Target"),
            param_key: p.get_str("ParamKey"),
            any: p.get_str("any"),
            hit_rate: p.get_i32("HitRate"),
            avoid_type: p.get_str("AvoidType"),
            relate_target: p.get_str("RelateTarget"),
            relate: p.get_str("Relate"),
            power: p.get_i32("Power"),
            reduce: p.get_str("Reduce"),
            can_crit: p.get_bool("CanCrit"),
            speed: p.get_i32("Speed"),
            delay: p.get_i32("Delay"),
            skill_tag: p.get_str("SkillTag"),
            cond: p.get_str("Cond"),
            free_val: p.get_str("FreeVal"),
            tag: p.get_str("Tag"),
            freq: p.get_i32("Freq"),
            inc_target: p.get_str("IncTarget"),
            inc_relate: p.get_str("IncRelate"),
            inc_power: p.get_i32("IncPower"),
            state_last: last,
            act_num: p.get_i32("ActNum"),
            crit_rate: p.get_i32("CritRate"),
            is_skill: p.get_bool("IsSkill"),
            check_target: p.get_bool("CheckTarget"),
            any_ref,
        }
    }
}
