use std::collections::{BTreeSet, HashMap, HashSet};
use std::convert::TryInto;
use std::hash::{BuildHasher, Hash, Hasher};

use json::JsonValue;
use yaml_rust::{Yaml, YamlLoader};

use data::skill::{Act, ActNode, AvoidType, ParamKey, Skill, SkillCategory, SkillMap, SkillMode};
use data::Sprite;
use idhash::IdHash;
use table::Table;

struct SkillRow<'a> {
    row_id: &'a str,
    name: &'a str,
    id: &'a str,
    order: usize,
    icon: &'a str,
    category: SkillCategory,
    poss_num: i8,
    for_user: bool,
    on_dict: bool,
    rarity: i8,
    freq: i8,
    skill_mode: &'a str,
    aff1: i8,
    aff2: i8,
    aff3: i8,
    aff4: i8,
    audio: &'a str,
    in_dict: bool,
    drop: bool,
    tag: &'a str,
    is_free: bool,
    seed: usize,
    enable: &'a str,
}

impl<'a> SkillRow<'a> {
    fn new(e: &'a HashMap<String, JsonValue>) -> Self {
        Self {
            row_id: e.get("_row_id").unwrap().as_str().unwrap(),
            name: e.get("name").unwrap().as_str().unwrap(),
            id: e.get("ID").unwrap().as_str().unwrap(),
            order: e.get("Order").unwrap().as_str().unwrap().parse::<usize>().unwrap(),
            icon: e.get("Icon").unwrap().as_str().unwrap(),
            category: SkillCategory::from_str(e.get("Category").unwrap().as_str().unwrap()).unwrap(),
            poss_num: e.get("PossNum").unwrap().as_str().unwrap().parse::<i8>().unwrap(),
            for_user: str_to_bool(e.get("ForUser").unwrap().as_str().unwrap()),
            on_dict: str_to_bool(e.get("OnDict").unwrap().as_str().unwrap()),
            rarity: e.get("Rarity").unwrap().as_str().unwrap().parse::<i8>().unwrap(),
            freq: e.get("Freq").unwrap().as_str().unwrap().parse::<i8>().unwrap(),
            skill_mode: e.get("skill_mode").unwrap().as_str().unwrap(),
            aff1: e.get("Aff1").unwrap().as_str().unwrap().parse::<i8>().unwrap(),
            aff2: e.get("Aff2").unwrap().as_str().unwrap().parse::<i8>().unwrap(),
            aff3: e.get("Aff3").unwrap().as_str().unwrap().parse::<i8>().unwrap(),
            aff4: e.get("Aff4").unwrap().as_str().unwrap().parse::<i8>().unwrap(),
            audio: e.get("audio").unwrap().as_str().unwrap(),
            in_dict: str_to_bool(e.get("InDict").unwrap().as_str().unwrap()),
            drop: str_to_bool(e.get("Drop").unwrap().as_str().unwrap()),
            tag: e.get("Tag").unwrap().as_str().unwrap(),
            is_free: str_to_bool(e.get("IsFree").unwrap().as_str().unwrap()),
            seed: e.get("Seed").unwrap().as_str().unwrap().parse::<usize>().unwrap(),
            enable: e.get("Enable").unwrap().as_str().unwrap(),
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
    act_num: u8,
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
            act_num: e["ActNum"].as_str().unwrap().parse::<u8>().unwrap(),
        }
    }
}

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


fn str_to_bool(v: &str) -> bool {
    match v {
        "0" => false,
        "1" => true,
        _ => panic!("invalid bool value: {}", v),
    }
}

struct SkillWithId<'a> {
    skill: Skill,
    row: SkillRow<'a>,
}

impl Hash for SkillWithId<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.id.hash(state);
    }
}

pub fn process_skill(skill_table: &Table, skill_mode_table: &Table, act_table: &Table, act_node_table: &Table) {
    let act_node_entities = act_node_table.entities();
    let mut act_node_rows = act_node_entities.iter().map(|e| {
        ActNodeRow::new(e)
    }).collect::<Vec<_>>();
    act_node_rows.sort_by_key(|row| row.order);

    let act_entities = act_table.entities();
    let act_rows = act_entities.iter().map(|e| {
        ActRow::new(e)
    }).collect::<Vec<_>>();

    assert_eq!(skill_mode_table.id(), "E6p/0cim2Ui4oFyQYHe+8w");
    let skill_mode_entities = skill_mode_table.entities();
    let mut skill_mode_rows = skill_mode_entities.iter().map(|e| {
        SkillModeRow::new(e)
    }).collect::<Vec<_>>();
    skill_mode_rows.sort_by_key(|row| row.order);

    let skill_entities = skill_table.entities();
    let mut skills = skill_entities.iter().flat_map(|e| {
        let skill_row = SkillRow::new(e);

        let mode_rows = skill_mode_rows.iter().filter(|row| {
            row.skill == format!("{}_{}", skill_row.name, skill_row.row_id)
        }).collect::<Vec<_>>();

        let mut mode_categories = HashSet::new();

        let modes = mode_rows.iter().map(|mode_row| {
            let acts = act_rows.iter().filter(|act_row| {
                act_row.namer == format!("skill_mode.{}_{}_{}", mode_row.inner_name, skill_mode_table.id(), mode_row.row_id)
            }).map(|act_row| {
                let nodes = act_node_rows.iter().filter(|act_node_row| {
                    act_node_row.act == format!("{}_{}", act_row.name, act_row.row_id)
                }).filter(|row| row.action_type != "Visual").map(|act_node_row| {
                    println!("act_node_row: {:?}", act_node_row);
                    ActNode {
                        id: act_node_row.id.to_string(),
                        action_type: act_node_row.action_type.to_string(),
                        target: act_node_row.target,
                        param_key: ParamKey::from_str(act_node_row.param_key).unwrap(),
                        avoid_type: AvoidType::from_str(act_node_row.avoid_type).unwrap(),
                        act_num: act_node_row.act_num,
                    }
                }).collect::<Vec<_>>();
                println!("act {} has {:?} nodes", act_row.name, nodes);

                Act {
                    id: act_row.id.to_string(),
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
                acts
            }
        }).collect::<Vec<_>>();

        assert!(modes.len() > 0, "skill {} has no modes", skill_row.name);
        assert_eq!(mode_categories.len(), 1, "skill {} has multiple categories: {:?}", skill_row.name, mode_categories);

        let skill = Skill {
            hash: 0,
            id: skill_row.id.to_string(),
            category: skill_row.category.clone(),
            modes
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

    let mut skill_map = SkillMap::new();
    for skill in skills.iter() {
        skill_map.insert(skill.skill.hash, skill.skill.clone());
    }

    let file_writer = std::io::BufWriter::new(std::fs::File::create(format!("public/data/skill.avro")).unwrap());
    SkillMap::write(file_writer, &skill_map).unwrap();
}

fn parse_icon(name: &str) -> Sprite {
    let path = format!("dump/asset/ExportedProject/Assets/Sprite/{}.asset", name);
    let s = std::fs::read_to_string(path).unwrap();
    let docs = YamlLoader::load_from_str(s.as_str()).unwrap();
    let doc = &docs[0];
    let texture = &doc["Sprite"]["m_RD"]["texture"];
    assert_eq!(texture["guid"].as_str().unwrap(), "a50549b8827f09843841d13f031f165f");
    let rect = &doc["Sprite"]["m_Rect"];
    let x: Result<u16, _> = parse_number(&rect["x"]).try_into();
    let y: Result<u16, _> = parse_number(&rect["y"]).try_into();
    let width: Result<u8, _> = parse_number(&rect["width"]).try_into();
    let height: Result<u8, _> = parse_number(&rect["height"]).try_into();
    Sprite {
        x: x.unwrap(),
        y: y.unwrap(),
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