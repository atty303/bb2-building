extern crate ahash;
extern crate clap;
extern crate data;
extern crate json;
extern crate regex;
extern crate yaml_rust;

use clap::{Parser, Subcommand};
use data::LANGUAGES;
use json::JsonValue;

use skill::process_skill;
use state::state_repository_from_dump;
use table::act::ActTable;
use table::act_node::ActNodeTable;
use table::skill::SkillTable;
use table::skill_mode::SkillModeTable;
use table::sm_act::SmActTable;
use table::state::StateTable;
use table::{Table, UnknownTable};

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
    Term,
    Table,
    Skill {
        #[arg(long, default_value_t = false)]
        write: bool,
    },
    State,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Term => run_term(),

        Commands::Table => {
            let db = read_db();
            for meta in db["Metas"].members() {
                let table: Table<UnknownTable> = Table::new(meta.to_owned());
                table.to_csv(std::io::BufWriter::new(
                    std::fs::File::create(format!("dump/table/{}.csv", table.name())).unwrap(),
                ));
            }
        }
        Commands::Skill { write } => run_skill(write),
        Commands::State => {
            let mut state_table: Option<Table<StateTable>> = None;

            let db = read_db();
            for meta in db["Metas"].members() {
                let name = meta["Name"].as_str().unwrap();
                match name {
                    "state" => state_table = Some(Table::new(meta.to_owned())),
                    _ => (),
                }
            }

            state_repository_from_dump(&state_table.unwrap());
        }
    }
}

fn read_db() -> JsonValue {
    let s = std::fs::read_to_string("dump/db.json").unwrap();
    json::parse(s.as_str()).unwrap()
}

fn run_term() {
    let terms_i18n = terms::term_repository_from_dump();
    for lang in LANGUAGES {
        let mut writer = std::io::BufWriter::new(
            std::fs::File::create(format!("public/i18n/{}/term.msgpack", lang)).unwrap(),
        );
        let terms = terms_i18n.get(lang).unwrap();
        terms.write(&mut writer).unwrap();
    }
}

fn run_skill(write: bool) {
    let mut act_table: Option<Table<ActTable>> = None;
    let mut act_node_table: Option<Table<ActNodeTable>> = None;
    let mut skill_table: Option<Table<SkillTable>> = None;
    let mut skill_mode_table: Option<Table<SkillModeTable>> = None;
    let mut sm_act_table: Option<Table<SmActTable>> = None;
    let mut state_table: Option<Table<StateTable>> = None;

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
            _ => (),
        }
    }

    let skill_table = skill_table.unwrap();
    let skill_mode_table = skill_mode_table.unwrap();
    let sm_act_table = sm_act_table.unwrap();
    let act_table = act_table.unwrap();
    let act_node_table = act_node_table.unwrap();

    let states = state_repository_from_dump(&state_table.unwrap());
    let terms_i18n = terms::term_repository_from_dump();

    for lang in LANGUAGES {
        let terms = terms_i18n.get(lang).unwrap();
        let repo = process_skill(
            &skill_table,
            &skill_mode_table,
            &sm_act_table,
            &act_table,
            &act_node_table,
            terms,
            &states,
        );
        if write {
            let mut writer = std::io::BufWriter::new(
                std::fs::File::create(format!("public/i18n/{}/skill.msgpack", lang)).unwrap(),
            );
            repo.write(&mut writer).unwrap();
        }
    }
}
