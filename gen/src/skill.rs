use std::collections::HashSet;
use std::convert::TryInto;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use data::skill::{
    Act, ActNode, ActTrigger, AvoidType, ParamKey, Reduce, Skill, SkillCategory, SkillMode,
    SkillRepository, StateLast, Target,
};
use idhash::IdHash;
use sprite::parse_icon;
use table::act::ActTable;
use table::act_node::ActNodeTable;
use table::skill::SkillTable;
use table::skill_mode::SkillModeTable;
use table::sm_act::SmActTable;
use table::Table;

struct SkillWithId {
    skill: Skill,
    id: String,
    order: usize,
}

impl Hash for SkillWithId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub fn process_skill(
    skill_table: &Table<SkillTable>,
    skill_mode_table: &Table<SkillModeTable>,
    sm_act_table: &Table<SmActTable>,
    act_table: &Table<ActTable>,
    act_node_table: &Table<ActNodeTable>,
    write: bool,
) {
    let mut skills = skill_table
        .iter()
        .flat_map(|skill_row| {
            let mode_rows = skill_mode_table
                .iter()
                .filter(|row| row.skill == format!("{}_{}", skill_row.name, skill_row.row_id))
                .collect::<Vec<_>>();

            let mut mode_categories = HashSet::new();

            let modes = mode_rows
                .iter()
                .map(|mode_row| {
                    let sm_acts = sm_act_table
                        .iter()
                        .filter(|sm_act_row| {
                            sm_act_row.skill_mode
                                == format!("{}_{}", mode_row.name, mode_row.row_id)
                        })
                        .collect::<Vec<_>>();
                    assert!(
                        sm_acts.len() > 0,
                        "skill_mode {} has no sm_acts",
                        mode_row.name
                    );

                    let acts = sm_acts
                        .iter()
                        .map(|sm_act_row| {
                            let act_rows = act_table
                                .iter()
                                .filter(|act_row| {
                                    sm_act_row.act == format!("{}_{}", act_row.name, act_row.row_id)
                                })
                                .collect::<Vec<_>>();
                            assert_eq!(
                                act_rows.len(),
                                1,
                                "sm_act {} has multiple acts",
                                sm_act_row.name
                            );

                            let act_row = &act_rows[0];
                            let nodes = act_node_table
                                .iter()
                                .filter(|act_node_row| {
                                    act_node_row.act
                                        == format!("{}_{}", act_row.name, act_row.row_id)
                                })
                                .filter(|row| row.action_type != "Visual")
                                .map(|act_node_row| {
                                    // println!("act_node: {:?}", act_node_row);

                                    let last =
                                        act_node_row.state_last.split('|').collect::<Vec<_>>();
                                    assert_eq!(
                                        last.len(),
                                        5,
                                        "invalid state_last: {}",
                                        act_node_row.state_last
                                    );
                                    let last = last
                                        .iter()
                                        .map(|v| v.parse::<i8>().unwrap())
                                        .collect::<Vec<_>>();

                                    let state_row_id = if act_node_row.any.starts_with("state.") {
                                        Some(
                                            act_node_row
                                                .any
                                                .splitn(3, '_')
                                                .skip(2)
                                                .next()
                                                .unwrap()
                                                .to_string(),
                                        )
                                    } else {
                                        None
                                    };

                                    ActNode {
                                        id: act_node_row.id.to_string(),
                                        action_type: act_node_row.action_type.to_string(),
                                        target: act_node_row.target.try_into().unwrap(),
                                        param_key: ParamKey::from_str(&act_node_row.param_key)
                                            .unwrap(),
                                        state_row_id,
                                        hit_rate: act_node_row.hit_rate.try_into().unwrap(),
                                        avoid_type: AvoidType::from_str(&act_node_row.avoid_type)
                                            .unwrap(),
                                        relate_target: Target::from_str(
                                            &act_node_row.relate_target,
                                        )
                                        .unwrap(),
                                        relate: act_node_row.relate.to_string(),
                                        power: act_node_row.power.try_into().unwrap(),
                                        reduce: Reduce::from_str(&act_node_row.reduce).unwrap(),
                                        inc_target: Target::from_str(&act_node_row.inc_target)
                                            .unwrap(),
                                        inc_relate: act_node_row.inc_relate.to_string(),
                                        inc_power: act_node_row.inc_power.try_into().unwrap(),
                                        state_last: StateLast {
                                            f1: last[0],
                                            f2: last[1],
                                            f3: last[2],
                                            room: last[3],
                                            f5: last[4],
                                        },
                                        act_num: act_node_row.act_num.try_into().unwrap(),
                                        crit_rate: act_node_row.crit_rate.try_into().unwrap(),
                                    }
                                })
                                .collect::<Vec<_>>();

                            Act {
                                id: act_row.id.to_string(),
                                act_trigger: ActTrigger::from_str(&sm_act_row.act_trigger)
                                    .expect("act_trigger"),
                                nodes,
                            }
                        })
                        .collect::<Vec<_>>();

                    mode_categories.insert(mode_row.category.clone());

                    let icon = parse_icon(&mode_row.icon);

                    SkillMode {
                        id: mode_row.id.to_string(),
                        icon,
                        is_alt: mode_row.alt_mode,
                        is_brave: mode_row.is_brave,
                        use_num: mode_row.use_num.try_into().unwrap(),
                        use_brave: mode_row.use_brave.try_into().unwrap(),
                        cooldown: mode_row.cooldown.try_into().unwrap(),
                        use_init: mode_row.use_init,
                        is_quick: mode_row.is_quick,
                        acts,
                        poss_num: skill_row.poss_num.try_into().unwrap(),
                    }
                })
                .collect::<Vec<_>>();

            assert!(modes.len() > 0, "skill {} has no modes", skill_row.name);
            assert_eq!(
                mode_categories.len(),
                1,
                "skill {} has multiple categories: {:?}",
                skill_row.name,
                mode_categories
            );

            if skill_row.enable.is_empty() && skill_row.in_dict {
                let skill = Skill {
                    hash: 0,
                    id: skill_row.id.to_string(),
                    modes,
                    category: SkillCategory::from_str(skill_row.category.as_str()).unwrap(),
                    rarity: skill_row.rarity.try_into().unwrap(),
                    in_dictionary: skill_row.in_dict,
                    is_free: skill_row.is_free,
                };
                Some(SkillWithId {
                    skill,
                    id: skill_row.id.clone(),
                    order: skill_row.order,
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Search for a seed that will produce unique ids for all skills
    let mut id_hasher = IdHash::new(0, 16);
    id_hasher.search_seed(&skills);
    assert_eq!(id_hasher.seed, 0);
    for skill in &mut skills {
        skill.skill.hash = id_hasher.id_hash(&skill) as u16;
    }

    skills.sort_by_key(|s| (!s.skill.is_free, s.order));

    if write {
        let file_writer = std::io::BufWriter::new(
            std::fs::File::create(format!("public/data/skill.avro")).unwrap(),
        );
        SkillRepository::write(file_writer, skills.iter().map(|s| &s.skill)).unwrap();
    }
}
