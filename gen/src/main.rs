extern crate ahash;
extern crate data;
extern crate json;
extern crate prettytable;
extern crate yaml_rust;
extern crate regex;

use std::collections::HashMap;

use skill::process_skill;
use state::process_state;
use table::{BGTable, Table};
use table::act::ActTable;
use table::act_node::ActNodeTable;
use table::skill::SkillTable;
use table::skill_mode::SkillModeTable;
use table::sm_act::SmActTable;
use table::state::StateTable;

mod terms;
mod table;
mod skill;
mod idhash;
mod state;
mod sprite;

fn main() {
    // terms::write_terms();

    let s = std::fs::read_to_string("dump/db.json").unwrap();
    let db_json = json::parse(s.as_str()).unwrap();

    let mut db = HashMap::new();
    for meta in db_json["Metas"].members() {
        let meta_name = meta["Name"].as_str().unwrap();
        let table = BGTable::new(meta);
        // table.to_csv(std::io::BufWriter::new(std::fs::File::create(format!("dump/table/{}.csv", meta_name)).unwrap()));

        // println!("{}: {}", meta_name, table.id());

        db.insert(meta_name.to_string(), table);
    }

    let mut act_table: Option<Table<ActTable>> = None;
    let mut act_node_table: Option<Table<ActNodeTable>> = None;
    let mut skill_table: Option<Table<SkillTable>> = None;
    let mut skill_mode_table: Option<Table<SkillModeTable>> = None;
    let mut sm_act_table: Option<Table<SmActTable>> = None;
    let mut state_table: Option<Table<StateTable>> = None;
    for meta in db_json["Metas"].members() {
        let name = meta["Name"].as_str().unwrap();
        match name {
            "act" => act_table = Some(Table::new(meta.to_owned())),
            "act_node" => act_node_table = Some(Table::new(meta.to_owned())),
            "skill" => skill_table = Some(Table::new(meta.to_owned())),
            "skill_mode" => skill_mode_table = Some(Table::new(meta.to_owned())),
            "sm_act" => sm_act_table = Some(Table::new(meta.to_owned())),
            "state" => state_table = Some(Table::new(meta.to_owned())),
            _ => ()
        }
    }

    process_skill(
        &skill_table.unwrap(),
        &skill_mode_table.unwrap(),
        &sm_act_table.unwrap(),
        &act_table.unwrap(),
        &act_node_table.unwrap());

    process_state(&state_table.unwrap());
}
