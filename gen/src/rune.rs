use data::term::TermRepository;
use data::{Rune, RuneRepository};
use idhash::IdHash;
use sprite::parse_icon;
use std::convert::TryInto;
use std::hash::{Hash, Hasher};
use table::rune::RuneTable;
use table::Table;

struct RuneWrapper(Rune);

impl Hash for RuneWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.id.hash(state);
    }
}

pub fn process_rune(rune_table: &Table<RuneTable>, terms: &TermRepository) -> RuneRepository {
    let mut runes = rune_table
        .iter()
        .flat_map(|rune_row| {
            if rune_row.enable.is_empty() && rune_row.in_dict {
                let rune = Rune {
                    hash: 0,
                    id: rune_row.id.clone(),
                    order: rune_row.order.try_into().unwrap(),
                    icon: parse_icon(&rune_row.icon),
                    rarity: rune_row.rarity.try_into().unwrap(),
                    name: terms.get_str(&format!("NM-{}", rune_row.id)),
                    description: terms.get(&format!("DC-{}", rune_row.id)),
                };
                Some(RuneWrapper(rune))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Search for a seed that will produce unique ids for all skills
    let mut id_hasher = IdHash::new(0, 16);
    id_hasher.search_seed(&runes);
    assert_eq!(id_hasher.seed, 0);
    for rune in &mut runes {
        rune.0.hash = id_hasher.id_hash(&rune) as u16;
    }
    runes.sort_by_key(|s| s.0.order);

    RuneRepository::from_vec(runes.iter().map(|r| r.0.clone()).collect::<Vec<_>>())
}
