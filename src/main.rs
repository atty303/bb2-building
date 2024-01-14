use data::{SkillCategory, SkillMap, TermMap};
use crate::components::app::App;

mod components;

fn main() {
    // let term_map = TermMap::read(std::io::BufReader::new(std::fs::File::open("public/i18n/ja/terms.avro").unwrap())).unwrap();
    // let skill_map = SkillMap::read(std::io::BufReader::new(std::fs::File::open("public/skill.avro").unwrap())).unwrap();
    //
    // for skill in skill_map.values().filter(|s| s.category != SkillCategory::Enemy) {
    //     println!("* {} ({:?})", term_map.get_name(&skill.modes[0].id), &skill.category);
    //     for mode in &skill.modes {
    //         println!("  + [{}{}{}{}] {} (-{} +{})",
    //                  if mode.is_alt { "A" } else { " " },
    //                  if mode.is_brave { "B" } else { " " } ,
    //                  if mode.is_quick { "Q" } else { " " },
    //                  if mode.use_init { "I" } else { "" },
    //                  term_map.get_name(&mode.id),
    //                  if mode.is_brave { mode.use_brave } else { mode.use_num },
    //                  mode.cooldown,
    //         );
    //
    //         for act in &mode.acts {
    //             println!("    - {}", term_map.get_name(&act.id));
    //             for node in &act.nodes {
    //                 if node.action_type == "Visual" {
    //                     continue;
    //                 }
    //                 println!("      - {}", term_map.get_action_type(&node.action_type));
    //             }
    //         }
    //     }
    // }

    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
}
