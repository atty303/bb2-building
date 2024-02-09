use data::term::TermRepository;
use data::GlobalRepository;

pub fn process_global(terms: &TermRepository) -> GlobalRepository {
    let mut rarity_colors = terms
        .iter()
        .filter(|(k, _)| k.starts_with("CLR-Star-Rarity-"))
        .map(|(k, v)| {
            let r = k.replace("CLR-Star-Rarity-", "").parse::<u8>().unwrap();
            (r, format!("{}", v.tokens))
        })
        .collect::<Vec<_>>();
    rarity_colors.sort_by(|(a, _), (b, _)| a.cmp(b));
    let rarity_colors = rarity_colors
        .iter()
        .map(|(_, v)| v.clone())
        .collect::<Vec<_>>();

    GlobalRepository { rarity_colors }
}
