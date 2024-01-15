use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use json::JsonValue;
use yaml_rust::{Yaml, YamlLoader};

use data::skill::{Act, ActNode, ActTrigger, AvoidType, ParamKey, Reduce, Skill, SkillCategory, SkillMode, SkillRepository, StateLast, Target};
use data::Sprite;
use idhash::IdHash;
use table::{BGTable, Table};
use table::skill::{SkillRow, SkillTable};

struct SkillModeRow<'a> {
    row_id: &'a str,
    id: &'a str,
    inner_name: &'a str,
    /// skill_mode(N) = skill(1) relation (format: `skill_mode.{}_{}_{}`)
    skill: &'a str,
    order: usize,
    icon: &'a str,
    category: SkillCategory,
    alt_mode: bool,
    is_brave: bool,
    use_num: i8,
    use_brave: i8,
    cooldown: i8,
    use_init: bool,
    is_quick: bool,
    sm_act: &'a str,
    skill_tag: &'a str,
}

impl<'a> SkillModeRow<'a> {
    fn new(e: &'a HashMap<String, JsonValue>) -> Self {
        Self {
            row_id: e["_row_id"].as_str().unwrap(),
            id: e["ID"].as_str().unwrap(),
            inner_name: e["name"].as_str().unwrap(),
            skill: e["skill"].as_str().unwrap(),
            order: e["Order"].as_str().unwrap().parse::<usize>().unwrap(),
            icon: e["Icon"].as_str().unwrap(),
            category: SkillCategory::from_str(e["Category"].as_str().unwrap()).unwrap(),
            alt_mode: str_to_bool(e["AltMode"].as_str().unwrap()),
            is_brave: str_to_bool(e["IsBrave"].as_str().unwrap()),
            use_num: e["UseNum"].as_str().unwrap().parse::<i8>().unwrap(),
            use_brave: e["UseBrave"].as_str().unwrap().parse::<i8>().unwrap(),
            cooldown: e["Cooldown"].as_str().unwrap().parse::<i8>().unwrap(),
            use_init: str_to_bool(e["UseInit"].as_str().unwrap()),
            is_quick: str_to_bool(e["IsQuick"].as_str().unwrap()),
            sm_act: e["sm_act"].as_str().unwrap(),
            skill_tag: e["SkillTag"].as_str().unwrap(),
        }
    }
}

struct SmActRow<'a> {
    row_id: &'a str,
    id: &'a str,
    name: &'a str,
    skill_mode: &'a str,
    act: &'a str,
    act_trigger: &'a str,
    freq: u8,
}

impl<'a> SmActRow<'a> {
    fn new(e: &'a HashMap<String, JsonValue>) -> Self {
        Self {
            row_id: e["_row_id"].as_str().unwrap(),
            id: e["ID"].as_str().unwrap(),
            name: e["name"].as_str().unwrap(),
            skill_mode: e["skill_mode"].as_str().unwrap(),
            act: e["act"].as_str().unwrap(),
            act_trigger: e["ActTrigger"].as_str().unwrap(),
            freq: e["Freq"].as_str().unwrap().parse::<u8>().unwrap(),
        }
    }
}

struct ActRow<'a> {
    row_id: &'a str,
    name: &'a str,
    id: &'a str,
    order: usize,
    act_node: &'a str,
    tag: &'a str,
    link_key: &'a str,
    is_rune: bool,
    namer: &'a str,
}

impl<'a> ActRow<'a> {
    fn new(e: &'a HashMap<String, JsonValue>) -> Self {
        Self {
            row_id: e.get("_row_id").unwrap().as_str().unwrap(),
            name: e.get("name").unwrap().as_str().unwrap(),
            id: e.get("ID").unwrap().as_str().unwrap(),
            order: e.get("Order").unwrap().as_str().unwrap().parse::<usize>().unwrap(),
            act_node: e.get("act_node").unwrap().as_str().unwrap(),
            tag: e.get("Tag").unwrap().as_str().unwrap(),
            link_key: e.get("LinkKey").unwrap().as_str().unwrap(),
            is_rune: str_to_bool(e.get("IsRune").unwrap().as_str().unwrap()),
            namer: e["namer"].as_str().unwrap(),
        }
    }
}

#[derive(Debug)]
struct ActNodeRow<'a> {
    row_id: &'a str,
    id: &'a str,
    inner_name: &'a str,
    order: usize,
    act: &'a str,
    p_order: usize,
    action_type: &'a str,
    target: i8,
    param_key: &'a str,
    any: &'a str,
    hit_rate: u16,
    avoid_type: &'a str,
    relate_target: &'a str,
    relate: &'a str,
    power: u32,
    reduce: &'a str,
    can_crit: bool,
    speed: u8,
    delay: u8,
    skill_tag: &'a str,
    cond: &'a str,
    free_val: &'a str,
    tag: &'a str,
    freq: u8,
    inc_target: &'a str,
    inc_relate: &'a str,
    inc_power: u16,
    state_last: &'a str,
    act_num: u8,
    crit_rate: u16,
    is_skill: bool,
    check_target: bool,
}

impl<'a> ActNodeRow<'a> {
    fn new(e: &'a HashMap<String, JsonValue>) -> Self {
        Self {
            row_id: e["_row_id"].as_str().unwrap(),
            id: e["ID"].as_str().unwrap(),
            inner_name: e["name"].as_str().unwrap(),
            order: e["Order"].as_str().unwrap().parse::<usize>().unwrap(),
            act: e["act"].as_str().unwrap(),
            p_order: e["POrder"].as_str().unwrap().parse::<usize>().unwrap(),
            action_type: e["ActionType"].as_str().unwrap(),
            target: e["Target"].as_str().unwrap().parse::<i8>().unwrap(),
            param_key: e["ParamKey"].as_str().unwrap(),
            any: e["any"].as_str().unwrap(),
            hit_rate: e["HitRate"].as_str().unwrap().parse::<u16>().unwrap(),
            avoid_type: e["AvoidType"].as_str().unwrap(),
            relate_target: e["RelateTarget"].as_str().unwrap(),
            relate: e["Relate"].as_str().unwrap(),
            power: e["Power"].as_str().unwrap().parse::<u32>().unwrap(),
            reduce: e["Reduce"].as_str().unwrap(),
            can_crit: str_to_bool(e["CanCrit"].as_str().unwrap()),
            speed: e["Speed"].as_str().unwrap().parse::<u8>().unwrap(),
            delay: e["Delay"].as_str().unwrap().parse::<u8>().unwrap(),
            skill_tag: e["SkillTag"].as_str().unwrap(),
            cond: e["Cond"].as_str().unwrap(),
            free_val: e["FreeVal"].as_str().unwrap(),
            tag: e["Tag"].as_str().unwrap(),
            freq: e["Freq"].as_str().unwrap().parse::<u8>().unwrap(),
            inc_target: e["IncTarget"].as_str().unwrap(),
            inc_relate: e["IncRelate"].as_str().unwrap(),
            inc_power: e["IncPower"].as_str().unwrap().parse::<u16>().unwrap(),
            state_last: e["StateLast"].as_str().unwrap(),
            act_num: e["ActNum"].as_str().unwrap().parse::<u8>().unwrap(),
            crit_rate: e["CritRate"].as_str().unwrap().parse::<u16>().unwrap(),
            is_skill: str_to_bool(e["IsSkill"].as_str().unwrap()),
            check_target: str_to_bool(e["CheckTarget"].as_str().unwrap()),
        }
    }
}



fn str_to_bool(v: &str) -> bool {
    match v {
        "0" => false,
        "1" => true,
        _ => panic!("invalid bool value: {}", v),
    }
}

struct SkillWithId {
    skill: Skill,
    row: SkillRow,
}

impl Hash for SkillWithId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.id.hash(state);
    }
}

pub fn process_skill(skill_table: &Table<SkillTable>, skill_mode_table: &BGTable, sm_act_table: &BGTable, act_table: &BGTable, act_node_table: &BGTable) {
    assert_eq!(skill_mode_table.id(), "E6p/0cim2Ui4oFyQYHe+8w");
    let skill_mode_entities = skill_mode_table.entities();
    let mut skill_mode_rows = skill_mode_entities.iter().map(|e| {
        SkillModeRow::new(e)
    }).collect::<Vec<_>>();
    skill_mode_rows.sort_by_key(|row| row.order);

    let sm_act_entities = sm_act_table.entities();
    let sm_act_rows = sm_act_entities.iter().map(|e| {
        SmActRow::new(e)
    }).collect::<Vec<_>>();

    let act_node_entities = act_node_table.entities();
    let mut act_node_rows = act_node_entities.iter().map(|e| {
        ActNodeRow::new(e)
    }).collect::<Vec<_>>();
    act_node_rows.sort_by_key(|row| row.order);

    let act_entities = act_table.entities();
    let act_rows = act_entities.iter().map(|e| {
        ActRow::new(e)
    }).collect::<Vec<_>>();

    let mut skills = skill_table.iter().flat_map(|skill_row| {
        let mode_rows = skill_mode_rows.iter().filter(|row| {
            row.skill == format!("{}_{}", skill_row.name, skill_row.row_id)
        }).collect::<Vec<_>>();

        let mut mode_categories = HashSet::new();

        let modes = mode_rows.iter().map(|mode_row| {
            let sm_acts = sm_act_rows.iter().filter(|sm_act_row| {
                sm_act_row.skill_mode == format!("{}_{}", mode_row.inner_name, mode_row.row_id)
            }).collect::<Vec<_>>();
            assert!(sm_acts.len() > 0, "skill_mode {} has no sm_acts", mode_row.inner_name);

            let acts = sm_acts.iter().map(|sm_act_row| {
                let act_rows = act_rows.iter().filter(|act_row| {
                    sm_act_row.act == format!("{}_{}", act_row.name, act_row.row_id)
                }).collect::<Vec<_>>();
                assert_eq!(act_rows.len(), 1, "sm_act {} has multiple acts", sm_act_row.name);

                let act_row = &act_rows[0];
                let nodes = act_node_rows.iter().filter(|act_node_row| {
                    act_node_row.act == format!("{}_{}", act_row.name, act_row.row_id)
                }).filter(|row| row.action_type != "Visual").map(|act_node_row| {
                    // println!("act_node: {:?}", act_node_row);

                    let last = act_node_row.state_last.split('|').collect::<Vec<_>>();
                    assert_eq!(last.len(), 5, "invalid state_last: {}", act_node_row.state_last);
                    let last = last.iter().map(|v| v.parse::<i8>().unwrap()).collect::<Vec<_>>();

                    let state_row_id = if act_node_row.any.starts_with("state.") {
                        Some(act_node_row.any.splitn(3, '_').skip(2).next().unwrap().to_string())
                    } else {
                        None
                    };

                    ActNode {
                        id: act_node_row.id.to_string(),
                        action_type: act_node_row.action_type.to_string(),
                        target: act_node_row.target,
                        param_key: ParamKey::from_str(act_node_row.param_key).unwrap(),
                        state_row_id,
                        hit_rate: act_node_row.hit_rate,
                        avoid_type: AvoidType::from_str(act_node_row.avoid_type).unwrap(),
                        relate_target: Target::from_str(act_node_row.relate_target).unwrap(),
                        relate: act_node_row.relate.to_string(),
                        power: act_node_row.power,
                        reduce: Reduce::from_str(act_node_row.reduce).unwrap(),
                        inc_target: Target::from_str(act_node_row.inc_target).unwrap(),
                        inc_relate: act_node_row.inc_relate.to_string(),
                        inc_power: act_node_row.inc_power,
                        state_last: StateLast { f1: last[0], f2: last[1], f3: last[2], room: last[3], f5: last[4] },
                        act_num: act_node_row.act_num,
                        crit_rate: act_node_row.crit_rate,
                    }
                }).collect::<Vec<_>>();

                Act {
                    id: act_row.id.to_string(),
                    act_trigger: ActTrigger::from_str(sm_act_row.act_trigger).expect("act_trigger"),
                    nodes
                }
            }).collect::<Vec<_>>();

            mode_categories.insert(mode_row.category.clone());

            let icon = parse_icon(mode_row.icon);

            SkillMode {
                id: mode_row.id.to_string(),
                icon,
                is_alt: mode_row.alt_mode,
                is_brave: mode_row.is_brave,
                use_num: mode_row.use_num,
                use_brave: mode_row.use_brave,
                cooldown: mode_row.cooldown,
                use_init: mode_row.use_init,
                is_quick: mode_row.is_quick,
                acts,
                poss_num: skill_row.poss_num.try_into().unwrap(),
            }
        }).collect::<Vec<_>>();

        assert!(modes.len() > 0, "skill {} has no modes", skill_row.name);
        assert_eq!(mode_categories.len(), 1, "skill {} has multiple categories: {:?}", skill_row.name, mode_categories);

        let skill = Skill {
            hash: 0,
            id: skill_row.id.to_string(),
            modes,
            category: SkillCategory::from_str(skill_row.category.as_str()).unwrap(),
            rarity: skill_row.rarity.try_into().unwrap(),
            in_dictionary: skill_row.in_dict,
            is_free: skill_row.is_free,
        };

        Some(SkillWithId { skill, row : skill_row })
    }).collect::<Vec<_>>();

    // Search for a seed that will produce unique ids for all skills
    let mut id_hasher = IdHash::new(0, 16);
    id_hasher.search_seed(&skills);
    assert_eq!(id_hasher.seed, 0);
    for skill in &mut skills {
        skill.skill.hash = id_hasher.id_hash(&skill) as u16;
    }

    skills.sort_by_key(|s| (!s.skill.is_free, s.row.order));

    let file_writer = std::io::BufWriter::new(std::fs::File::create(format!("public/data/skill.avro")).unwrap());
    SkillRepository::write(file_writer, skills.iter().map(|s| &s.skill)).unwrap();
}

fn parse_icon(name: &str) -> Sprite {
    let path = format!("dump/asset/ExportedProject/Assets/Sprite/{}.asset", name);
    let s = std::fs::read_to_string(path).unwrap();
    let docs = YamlLoader::load_from_str(s.as_str()).unwrap();
    let doc = &docs[0];
    let texture = &doc["Sprite"]["m_RD"]["texture"];
    assert_eq!(texture["guid"].as_str().unwrap(), "a50549b8827f09843841d13f031f165f");
    let texture_height = 4096;
    let rect = &doc["Sprite"]["m_Rect"];
    let x: Result<u16, _> = parse_number(&rect["x"]).try_into();
    let y: Result<u16, _> = parse_number(&rect["y"]).try_into();
    let width: Result<u8, _> = parse_number(&rect["width"]).try_into();
    let height: Result<u8, _> = parse_number(&rect["height"]).try_into();
    Sprite {
        x: x.unwrap(),
        y: texture_height - y.unwrap() - height.unwrap() as u16,
        width: width.unwrap(),
        height: height.unwrap(),
    }
}

fn parse_number(v: &Yaml) -> u64 {
    if let Some(i) = v.as_i64() {
        i as u64
    } else if let Some(f) = v.as_f64() {
        f.round() as u64
    } else {
        panic!("invalid number: {:?}", v);
    }
}