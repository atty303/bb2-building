extern crate ahash;
extern crate clap;
extern crate data;
extern crate json;
extern crate regex;
extern crate serde;
extern crate yaml_rust;

use clap::{Parser, Subcommand};
use json::JsonValue;
use std::rc::Rc;

use data::LANGUAGES;
use state::state_repository_from_dump;
use table::act::ActTable;
use table::act_node::ActNodeTable;
use table::enemy::EnemyTable;
use table::skill::SkillTable;
use table::skill_mode::SkillModeTable;
use table::sm_act::SmActTable;
use table::state::StateTable;
use table::{Table, UnknownTable};

mod global;
mod idhash;
mod skill;
mod sprite;
mod state;
mod table;
mod terms;

#[derive(Debug, Parser)]
#[command(name = "gen")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Table,
    Database {
        #[arg(long, default_value = "all")]
        lang: String,
        #[arg(long, default_value_t = false)]
        write: bool,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Table => run_table(),
        Commands::Database { lang, write } => run_database(lang, write),
    }
}

fn read_db() -> JsonValue {
    let s = std::fs::read_to_string("dump/db.json").unwrap();
    json::parse(s.as_str()).unwrap()
}

fn run_table() {
    let db = read_db();
    for meta in db["Metas"].members() {
        let table: Table<UnknownTable> = Table::new(meta.to_owned());
        table.to_csv(std::io::BufWriter::new(
            std::fs::File::create(format!("dump/table/{}.csv", table.name())).unwrap(),
        ));
    }
}

fn run_database(lang: String, write: bool) {
    let mut act_table: Option<Table<ActTable>> = None;
    let mut act_node_table: Option<Table<ActNodeTable>> = None;
    let mut skill_table: Option<Table<SkillTable>> = None;
    let mut skill_mode_table: Option<Table<SkillModeTable>> = None;
    let mut sm_act_table: Option<Table<SmActTable>> = None;
    let mut state_table: Option<Table<StateTable>> = None;
    let mut enemy_table: Option<Table<EnemyTable>> = None;

    let db = read_db();
    for meta in db["Metas"].members() {
        let name = meta["Name"].as_str().unwrap();
        match name {
            "act" => act_table = Some(Table::new(meta.to_owned())),
            "act_node" => act_node_table = Some(Table::new(meta.to_owned())),
            "skill" => skill_table = Some(Table::new(meta.to_owned())),
            "skill_mode" => skill_mode_table = Some(Table::new(meta.to_owned())),
            "sm_act" => sm_act_table = Some(Table::new(meta.to_owned())),
            "state" => state_table = Some(Table::new(meta.to_owned())),
            "enemy" => enemy_table = Some(Table::new(meta.to_owned())),
            _ => (),
        }
    }

    let skill_table = skill_table.unwrap();
    let skill_mode_table = skill_mode_table.unwrap();
    let sm_act_table = sm_act_table.unwrap();
    let act_table = act_table.unwrap();
    let act_node_table = act_node_table.unwrap();
    let enemy_table = enemy_table.unwrap();

    let states = state_repository_from_dump(&state_table.unwrap());
    let terms_i18n = terms::term_repository_from_dump();

    let langs = if lang == "all" {
        LANGUAGES.to_vec()
    } else {
        vec![lang.as_str()]
    };
    for lang in langs {
        let terms = terms_i18n.get(lang).unwrap();

        let global = global::process_global(&terms);
        let skill = skill::process_skill(
            &skill_table,
            &skill_mode_table,
            &sm_act_table,
            &act_table,
            &act_node_table,
            &enemy_table,
            terms,
            &states,
        );
        let database = data::Database {
            global,
            term: Rc::new(terms.clone()),
            skill: Rc::new(skill),
        };

        if write {
            let mut writer = std::io::BufWriter::new(
                std::fs::File::create(format!("public/i18n/{}/database.msgpack", lang)).unwrap(),
            );
            database.write(&mut writer).unwrap();
        }
    }
}
