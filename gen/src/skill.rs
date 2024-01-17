use std::convert::TryInto;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use data::skill::{
    Act, ActNode, ActTrigger, AvoidType, ParamKey, Reduce, Skill, SkillCategory, SkillMode,
    SkillRepository, Target,
};
use data::state::StateRepository;
use data::term::TermRepository;
use data::token::{Token, Tokens};
use idhash::IdHash;
use sprite::parse_icon;
use table::act::ActTable;
use table::act_node::{ActNodeRow, ActNodeTable};
use table::skill::{SkillRow, SkillTable};
use table::skill_mode::{SkillModeRow, SkillModeTable};
use table::sm_act::{SmActRow, SmActTable};
use table::Table;

struct SkillIdOrder {
    skill: Skill,
    id: String,
    order: usize,
}

impl Hash for SkillIdOrder {
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
    terms: &TermRepository,
    states: &StateRepository,
) -> SkillRepository {
    let mut skills = skill_table
        .iter()
        .flat_map(|skill_row| {
            let mode_rows = skill_mode_table
                .iter()
                .filter(|row| row.skill == format!("{}_{}", skill_row.name, skill_row.row_id))
                .collect::<Vec<_>>();

            let modes = mode_rows
                .iter()
                .map(|mode_row| {
                    process_skill_mode(
                        mode_row,
                        skill_row,
                        sm_act_table,
                        act_table,
                        act_node_table,
                        terms,
                        states,
                    )
                })
                .collect::<Vec<_>>();

            assert!(modes.len() > 0, "skill {} has no modes", skill_row.name);

            let name_id = modes[0].id.clone();

            if skill_row.enable.is_empty() && skill_row.in_dict {
                let skill = Skill {
                    hash: 0,
                    id: skill_row.id.to_string(),
                    modes,
                    category: SkillCategory::from_str(skill_row.category.as_str()).unwrap(),
                    rarity: skill_row.rarity.try_into().unwrap(),
                    in_dictionary: skill_row.in_dict,
                    is_free: skill_row.is_free,
                    name: terms.get_str(&format!("NM-{}", name_id)),
                };
                Some(SkillIdOrder {
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

    SkillRepository::from_vec(skills.iter().map(|s| s.skill.clone()).collect::<Vec<_>>())
}

fn process_skill_mode(
    mode_row: &SkillModeRow,
    skill_row: &SkillRow,
    sm_act_table: &Table<SmActTable>,
    act_table: &Table<ActTable>,
    act_node_table: &Table<ActNodeTable>,
    terms: &TermRepository,
    states: &StateRepository,
) -> SkillMode {
    let sm_act_rows = sm_act_table
        .iter()
        .filter(|r| r.skill_mode == format!("{}_{}", mode_row.name, mode_row.row_id))
        .collect::<Vec<_>>();
    assert!(
        sm_act_rows.len() > 0,
        "skill_mode {} has no sm_acts",
        mode_row.name
    );

    let acts = sm_act_rows
        .iter()
        .map(|sm_act_row| process_sm_act(sm_act_row, act_table, act_node_table, terms, states))
        .collect::<Vec<_>>();

    // format
    let head = terms
        .get(if mode_row.alt_mode {
            "NM-SkillNodeDesc-ModeName-AltMode"
        } else {
            "NM-SkillNodeDesc-ModeName-Normal"
        })
        .map_var_1(|out| {
            if mode_row.is_brave {
                terms.get("NM-SkillNodeDesc-ModeName-ForBrave").write(out);
            } else {
                Token::Empty.write(out);
            }
        });

    let tail = if skill_row.is_free {
        terms.get("NM-TIPS_FreeSkill")
    } else {
        let mut tail = terms.get("WD-Cooldown");
        let out = &mut tail;

        Token::Text(format!(": {}", mode_row.cooldown)).write(out);
        Token::NewLine.write(out);

        terms.get("WD-SkillPossRemain").write(out);
        Token::Text(format!(": -{}/{}", mode_row.use_num, skill_row.poss_num)).write(out);

        if mode_row.use_init {
            Token::NewLine.write(out);
            terms.get("NM-TIPS_UseInit").write(out);
        }

        tail
    };

    SkillMode {
        id: mode_row.id.to_string(),
        icon: parse_icon(&mode_row.icon),
        is_alt: mode_row.alt_mode,
        is_brave: mode_row.is_brave,
        use_num: mode_row.use_num.try_into().unwrap(),
        use_brave: mode_row.use_brave.try_into().unwrap(),
        cooldown: mode_row.cooldown.try_into().unwrap(),
        use_init: mode_row.use_init,
        is_quick: mode_row.is_quick,
        acts,
        name: terms.get_str(&format!("NM-{}", mode_row.id)),
        description_head: head,
        description_tail: tail,
        poss_num: skill_row.poss_num.try_into().unwrap(),
    }
}

fn process_sm_act(
    sm_act_row: &SmActRow,
    act_table: &Table<ActTable>,
    act_node_table: &Table<ActNodeTable>,
    terms: &TermRepository,
    states: &StateRepository,
) -> Act {
    let act_rows = act_table
        .iter()
        .filter(|r| sm_act_row.act == format!("{}_{}", r.name, r.row_id))
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
        .filter(|r| r.act == format!("{}_{}", act_row.name, act_row.row_id))
        .filter(|row| row.action_type != "Visual")
        .map(|act_node_row| process_act_node(act_node_row, terms, states))
        .collect::<Vec<_>>();

    let tokens = terms.get(&format!(
        "NM-SkillNodeDesc-ActTrigger-{}",
        sm_act_row.act_trigger
    ));

    Act {
        id: act_row.id.to_string(),
        act_trigger: ActTrigger::from_str(&sm_act_row.act_trigger).expect("act_trigger"),
        nodes,
        description: tokens,
    }
}

fn act_node_formatter(
    name: &str,
    out: &mut Tokens,
    row: &ActNodeRow,
    terms: &TermRepository,
    states: &StateRepository,
) {
    match name {
        "lasthit" => match row.avoid_type.as_str() {
            "LastHit" => terms.get("DC-SkillNodeDesc-LastHit").write(out),
            _ => Token::Empty.write(out), // TODO: error handling?
        },
        "t" => {
            let target = if row.target < 0 { 0 } else { row.target };
            terms
                .get(&format!("DC-SkillNodeDesc-TargetName-{}", target))
                .write(out);
        }
        "tg" => match row.param_key.as_str() {
            "LastAutoUse" => Token::Empty.write(out),
            key => terms
                .get(&format!("DC-SkillNodeDesc-TargetSkill-{}", key))
                .write(out),
        },
        "dr" => terms.get("WD-DamageType-Direct").write(out),
        "accu" => match row.avoid_type.as_str() {
            "" | "LastHit" => {
                Token::Indent.write(out);
                terms.get("DC-SkillNodeDesc-AvoidType-").write(out);
            }
            "A" => {
                Token::Indent.write(out);
                terms.get("DC-SkillNodeDesc-AvoidType-A").write(out);
            }
            "C" => {
                Token::Indent.write(out);
                terms.get("DC-SkillNodeDesc-AvoidType-C").write(out);
            }
            _ => {
                Token::Panic(format!("invalid avoid_type: {}", row.avoid_type)).write(out);
            }
        },
        "hit" => Token::Text(row.hit_rate.to_string()).write(out),
        "crit" => {
            if row.crit_rate == 0 || row.crit_rate == 100 {
                Token::Empty.write(out);
            } else {
                Token::Indent.write(out);
                terms
                    .get("DC-SkillNodeDesc-CritRate")
                    .map_var_1(|out| Token::Text(row.crit_rate.to_string()).write(out))
                    .write(out);
            }
        }
        // reduce
        "rd" => match row.reduce.as_str() {
            "" => Token::Empty.write(out),
            "P" | "M" | "V" => {
                Token::Indent.write(out);
                terms
                    .get(&format!("DC-SkillNodeDesc-Reduce-{}", row.reduce))
                    .write(out);
            }
            _ => Token::Panic(format!("invalid reduce: {}", row.reduce)).write(out),
        },
        // [威力強化] <irt>の<irf><ipw>%
        "inc" => {
            if row.inc_relate.is_empty() {
                Token::Empty.write(out);
            } else {
                Token::Indent.write(out);
                terms.get("DC-SkillNodeDesc-AboutIncPower").write(out)
            }
        }
        // increase relate target
        "irt" => match row.inc_target.as_str() {
            "SELF" => terms.get("DC-SkillNodeDesc-TargetName-0").write(out),
            "TARGET" => terms.get("DC-SkillNodeDesc-TargetName-1").write(out),
            _ => Token::Panic(format!("inc_target[{}]", row.inc_target)).write(out),
        },
        "irf" => {
            //let pair = row.inc_relate.split(':').collect::<Vec<_>>();
            let key = &row.inc_relate;
            match terms.try_get(&format!("NM-{}", key)) {
                Some(s) => s.write(out),
                None => Token::Error(format!("irf[{}]", key)).write(out),
            }
        }
        "ipw" => Token::Text(row.inc_power.to_string()).write(out),
        "power" => terms.get("DC-SkillNodeDesc-AboutPower").write(out),
        // <rt>の<rf> <pw>%
        "pwd" => {
            if row.relate.is_empty() {
                Token::Empty.write(out);
            } else {
                terms.get("DC-SkillNodeDesc-AboutPowerDtl").write(out)
            }
        }
        "rt" => match row.inc_target.as_str() {
            "SELF" => terms.get("DC-SkillNodeDesc-TargetName-0").write(out),
            "TARGET" => terms.get("DC-SkillNodeDesc-TargetName-1").write(out),
            _ => Token::Panic(format!("inc_target[{}]", row.inc_target)).write(out),
        },
        "rf" => {
            if row.relate.contains('/') {
                let mut it = row.relate.split('/');
                let or = [it.next().unwrap(), it.next().unwrap()]
                    .iter()
                    .map(|s| {
                        let n = &s[3..4];
                        terms.get(&format!("NM-MainParam:{}", n))
                    })
                    .collect::<Vec<_>>();
                or[0].write(out);
                terms.get("WD-Relate-Or").write(out);
                or[1].clone().write(out);
            } else {
                let r = terms
                    .try_get(&format!("DC-SkillNodeDesc-Relate-{}", row.relate))
                    .or_else(|| terms.try_get(&format!("NM-{}", row.relate)));
                match r {
                    Some(t) => t.write(out),
                    None => Token::Error(format!("relate[{}]", row.relate)).write(out),
                };
            }
        }
        "pw" => Token::Text(row.power.to_string()).write(out),
        "ppw" => {
            if row.relate.is_empty() {
                Token::Text(row.power.to_string()).write(out);
            } else {
                Token::Empty.write(out);
            }
        }
        "last" => {
            fn item(name: &str, value: i32, terms: &TermRepository) -> Tokens {
                terms
                    .get(&format!("DC-SkillNodeDesc-Last{}", name))
                    .map_var_1(|out| Token::Text(value.to_string()).write(out))
            }

            let mut items = vec![];
            if row.state_last[0] >= 0 {
                items.push(item("Act", row.state_last[0], terms));
            }
            if row.state_last[1] >= 0 {
                items.push(item("Turn", row.state_last[1], terms));
            }
            if row.state_last[2] >= 0 {
                items.push(item("Combat", row.state_last[2], terms));
            }
            if row.state_last[3] >= 0 {
                items.push(item("Room", row.state_last[3], terms));
            }
            if row.state_last[4] >= 0 {
                items.push(item("Floor", row.state_last[4], terms));
            }

            if !items.is_empty() {
                Token::Indent.write(out);
                terms
                    .get("DC-SkillNodeDesc-LastCombine")
                    .map_var_1(|out| {
                        let mut first = true;
                        for i in &items {
                            if !first {
                                terms.get("DC-SkillNodeDesc-LastDivider").write(out);
                            }
                            i.write(out);
                            first = false;
                        }
                    })
                    .write(out);
            } else {
                Token::Empty.write(out);
            }
        }
        // Add: <lasthit><t>に<st><srpw>を付与<stpw><rd><inc><accu><crit><last>
        "st" => {
            if let Some(state_row_id) = &row.state_row_id {
                if let Some(state) = states.get(state_row_id) {
                    if let Some(text) = terms.try_get(&format!("NM-{}", &state.id)) {
                        text.write(out);
                    } else {
                        Token::Error(format!("state[{}]", state.id)).write(out);
                    }
                } else {
                    Token::Panic(format!("state not found: {}", state_row_id)).write(out);
                }
            }
        }
        // Add: <lasthit><t>に<st><srpw>を付与<stpw><rd><inc><accu><crit><last>
        "srpw" => {
            if !row.relate.is_empty() {
                Token::Empty.write(out);
            } else {
                if let Some(state_row_id) = &row.state_row_id {
                    if let Some(state) = states.get(state_row_id) {
                        // TODO: ex) 「ポイズン→猛毒」のときは表示しない
                        let text = state.format.replace("{v}", &row.power.to_string());
                        Token::Text(text).write(out);
                    } else {
                        Token::Panic(format!("state not found: {}", state_row_id)).write(out);
                    }
                }
            }
        }
        "stpw" => {
            if row.relate.is_empty() {
                Token::Empty.write(out);
            } else {
                Token::Indent.write(out);
                terms.get("DC-SkillNodeDesc-AboutPowerDtl").write(out)
            }
        }
        "md" => {
            if row.action_type == "AltMode" {
                if row.power == 0 {
                    terms.get("WD-SkillAltModeName-0").write(out);
                } else if row.power == 1 {
                    terms.get("WD-SkillAltModeName-1").write(out);
                } else {
                    Token::Panic(format!("invalid power {}", row.power)).write(out);
                }
            } else {
                Token::Panic(format!("invalid action_type {}", row.action_type)).write(out);
            }
        }
        _ => (),
    }
}

fn process_act_node(
    act_node_row: &ActNodeRow,
    terms: &TermRepository,
    states: &StateRepository,
) -> ActNode {
    let description = match act_node_row.action_type.as_str() {
        "Reveal" => Tokens::new(),
        action_type => {
            let description = terms
                .get(&format!("DC-SkillNodeDesc-{}", action_type))
                .format(|out, s| act_node_formatter(s, out, &act_node_row, terms, states));
            if act_node_row.act_num != 1 {
                terms.get("DC-SkillNodeDesc-MultipleCase").map_var_2(
                    |out| description.write(out),
                    |out| Token::Text(act_node_row.act_num.to_string()).write(out),
                )
            } else {
                description
            }
        }
    };

    for t in description.vec() {
        match t {
            Token::Panic(s) => panic!("panic: {}", s),
            _ => (),
        }
    }

    ActNode {
        id: act_node_row.id.to_string(),
        action_type: act_node_row.action_type.to_string(),
        target: act_node_row.target.try_into().unwrap(),
        param_key: ParamKey::from_str(&act_node_row.param_key).unwrap(),
        state_row_id: act_node_row.state_row_id.clone(),
        hit_rate: act_node_row.hit_rate.try_into().unwrap(),
        avoid_type: AvoidType::from_str(&act_node_row.avoid_type).unwrap(),
        relate_target: Target::from_str(&act_node_row.relate_target).unwrap(),
        relate: act_node_row.relate.to_string(),
        power: act_node_row.power.try_into().unwrap(),
        reduce: Reduce::from_str(&act_node_row.reduce).unwrap(),
        inc_target: Target::from_str(&act_node_row.inc_target).unwrap(),
        inc_relate: act_node_row.inc_relate.to_string(),
        inc_power: act_node_row.inc_power.try_into().unwrap(),
        act_num: act_node_row.act_num.try_into().unwrap(),
        crit_rate: act_node_row.crit_rate.try_into().unwrap(),

        description,
    }
}
