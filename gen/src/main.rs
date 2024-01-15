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
use table::skill::SkillTable;
use table::state::StateTable;

mod terms;
mod table;
mod skill;
mod idhash;
mod state;

fn main() {
    // terms::write_terms();

    let s = std::fs::read_to_string("dump/db.json").unwrap();
    let db_json = json::parse(s.as_str()).unwrap();

    let mut db = HashMap::new();
    for meta in db_json["Metas"].members() {
        let meta_name = meta["Name"].as_str().unwrap();
        let table = BGTable::new(meta);
        // table.to_csv(std::io::BufWriter::new(std::fs::File::create(format!("dump/table/{}.csv", meta_name)).unwrap()));

        println!("{}: {}", meta_name, table.id());

        db.insert(meta_name.to_string(), table);
    }

    let mut skill_table: Option<Table<SkillTable>> = None;
    let mut state_table: Option<Table<StateTable>> = None;
    for meta in db_json["Metas"].members() {
        let name = meta["Name"].as_str().unwrap();
        match name {
            "skill" => skill_table = Some(Table::new(meta.to_owned())),
            "state" => state_table = Some(Table::new(meta.to_owned())),
            _ => ()
        }
    }

    process_skill(
        &skill_table.unwrap(),
        &db["skill_mode"],
        &db["sm_act"],
        &db["act"],
        &db["act_node"]);

    process_state(&state_table.unwrap());
}
