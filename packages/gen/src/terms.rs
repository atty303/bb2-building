use std::collections::HashMap;
use std::sync::OnceLock;

use regex::{Captures, Regex};
use yaml_rust::YamlLoader;

use data::term::{Term, TermRepository};
use data::token::Tokens;
use data::LANGUAGES;

use crate::data::token::Token;

pub fn term_repository_from_dump() -> HashMap<&'static str, TermRepository> {
    let s =
        std::fs::read_to_string("dump/asset/ExportedProject/Assets/Resources/I2Languages.asset")
            .unwrap();
    let docs = YamlLoader::load_from_str(s.as_str()).unwrap();
    let doc = &docs[0];
    let terms = doc["MonoBehaviour"]["mSource"]["mTerms"].as_vec().unwrap();

    let mut outs = Vec::new();
    for _lang in LANGUAGES {
        outs.push(Vec::<(String, String)>::new());
    }

    for term in terms {
        let key = term["Term"].as_str().unwrap();
        let langs = term["Languages"].as_vec().unwrap();
        for (i, _) in LANGUAGES.iter().enumerate() {
            let value = langs[2 + i].as_str().unwrap();
            let value = if value == "$ja" {
                langs[2].as_str().unwrap()
            } else if value == "$en" {
                langs[3].as_str().unwrap()
            } else if value.starts_with('$') {
                panic!("unknown language: {}", value)
            } else {
                value
            };
            outs[i].push((key.to_string(), value.to_string()));
        }
    }

    let mut repos = HashMap::new();

    let re = Regex::new(r"\{\[(.+?)]}").unwrap();
    for (i, lang) in LANGUAGES.iter().enumerate() {
        let out = &outs[i];

        let mut new_out = Vec::<(String, String)>::new();
        for (key, value) in out.iter() {
            let n = re.replace_all(value, |r: &Captures| {
                let key_ref = &r[1];
                let substitute = &out.iter().find(|t| t.0 == key_ref);
                if let Some(&ref s) = substitute {
                    s.1.clone()
                } else {
                    println!("{}: '{}' not found", lang, key_ref);
                    value.to_string()
                }
            });
            new_out.push((key.clone(), n.to_string()));
        }

        {
            let file_writer = std::io::BufWriter::new(
                std::fs::File::create(format!("dump/{}.csv", lang)).unwrap(),
            );
            let mut csv_writer = csv::Writer::from_writer(file_writer);
            for (key, value) in new_out.iter() {
                csv_writer.write_record(&[key, &value]).unwrap();
            }
        }

        let nodes = new_out
            .iter()
            .map(|(key, value)| {
                (
                    key.clone(),
                    Term {
                        tokens: Tokens::from_vec(parse(value)),
                    },
                )
            })
            .collect::<Vec<_>>();

        repos.insert(*lang, TermRepository::from_vec(nodes.clone()));
    }

    repos
}

static RE: OnceLock<Regex> = OnceLock::new();

fn parse(s: &str) -> Vec<Token> {
    let re = RE.get_or_init(|| Regex::new(r"(__)|(<[^>]+>)|(\{[^}]+})").expect("regex"));

    let mut splits = vec![0];
    for caps in re.captures_iter(s) {
        let m = caps.get(0).unwrap();
        splits.push(m.start());
        splits.push(m.end());
    }
    splits.push(s.len());

    let mut start = 0usize;
    let mut tokens = vec![];
    for end in splits {
        if start < end {
            let span = &s[start..end];
            if span.starts_with('<') {
                tokens.push(Token::Var(s[start + 1..end - 1].to_string()));
            } else if span.starts_with('{') {
                tokens.push(Token::Var(s[start + 1..end - 1].to_string()));
            } else if span == "__" {
                tokens.push(Token::NewLine);
            } else {
                tokens.push(Token::Text(span.to_string()));
            }
            start = end;
        }
    }
    tokens
}

#[cfg(test)]
mod test {
    use terms::parse;

    use super::Token;

    #[test]
    fn test_parse() {
        assert_eq!(parse(""), vec![]);
        assert_eq!(parse("abc"), vec![Token::Text("abc".to_string())]);
        assert_eq!(parse("__"), vec![Token::NewLine]);
        assert_eq!(parse("<abc>"), vec![Token::Var("abc".to_string())]);
        assert_eq!(
            parse("<abc><abc>"),
            vec![Token::Var("abc".to_string()), Token::Var("abc".to_string())]
        );
        assert_eq!(parse("{abc}"), vec![Token::Var("abc".to_string())]);
        assert_eq!(
            parse("{abc}{abc}"),
            vec![Token::Var("abc".to_string()), Token::Var("abc".to_string())]
        );
        assert_eq!(
            parse("abc__def"),
            vec![
                Token::Text("abc".to_string()),
                Token::NewLine,
                Token::Text("def".to_string())
            ]
        );
        assert_eq!(
            parse("abc<def>ghi"),
            vec![
                Token::Text("abc".to_string()),
                Token::Var("def".to_string()),
                Token::Text("ghi".to_string())
            ]
        );
        assert_eq!(
            parse("abc{def}ghi"),
            vec![
                Token::Text("abc".to_string()),
                Token::Var("def".to_string()),
                Token::Text("ghi".to_string())
            ]
        );
        assert_eq!(
            parse("abc__def<ghi>{jkl}"),
            vec![
                Token::Text("abc".to_string()),
                Token::NewLine,
                Token::Text("def".to_string()),
                Token::Var("ghi".to_string()),
                Token::Var("jkl".to_string())
            ]
        );
    }
}
