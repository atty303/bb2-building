use std::sync::OnceLock;

use regex::Regex;
use yaml_rust::YamlLoader;

use data::LANGUAGES;
use data::term::{Term, TermMap};

use crate::data::token::Token;

pub fn write_terms() {
    let s = std::fs::read_to_string("dump/asset/ExportedProject/Assets/Resources/I2Languages.asset").unwrap();
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
            } else {
                value
            };
            outs[i].push((key.to_string(), value.to_string()));
        }
    }

    let re = Regex::new(r"\{\[(.+?)]}").unwrap();
    for (i, lang) in LANGUAGES.iter().enumerate() {
        let out = &outs[i];

        let mut new_out = Vec::<(String, String)>::new();
        for (key, value) in out.iter() {
            if let Some(m) = re.captures(value) {
                let key_ref = &m[1].to_string();
                let substitute = &out.iter().find(|t| t.0 == key_ref.as_str());
                if let Some(&ref s) = substitute {
                    new_out.push(s.clone());
                } else {
                    println!("{}: '{}' not found", lang, key_ref);
                    new_out.push((key.clone(), value.clone()));
                }
            } else {
                new_out.push((key.clone(), value.clone()));
            }
        }

        {
            let file_writer = std::io::BufWriter::new(std::fs::File::create(format!("dump/{}.csv", lang)).unwrap());
            let mut csv_writer = csv::Writer::from_writer(file_writer);
            for (key, value) in new_out.iter() {
                csv_writer.write_record(&[key, &value]).unwrap();
            }
        }

        let nodes = new_out.iter().map(|(key, value)| {
            let nodes = parse(value);
            (key.clone(), Term { nodes })
        }).collect::<Vec<_>>();

        let file_writer = std::io::BufWriter::new(std::fs::File::create(format!("public/i18n/{}/term.avro", lang)).unwrap());
        TermMap::write(file_writer, nodes.iter()).unwrap();

    }
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

    let mut at = 0usize;
    let mut nodes = vec![];
    for end in splits {
        if at < end {
            let span = &s[at..end];
            if span.starts_with("<") {
                nodes.push(Token::Var(s[at + 1..end - 1].to_string()));
            } else if span.starts_with("{") {
                nodes.push(Token::Var(s[at + 1..end - 1].to_string()));
            } else if span == "__" {
                nodes.push(Token::NewLine);
            } else {
                nodes.push(Token::Text(span.to_string()));
            }
            at = end;
        }
    }
    nodes
}

#[cfg(test)]
mod test {
    use terms::parse;

    use super::Token;

    #[test]
    fn test_parse() {
        assert_eq!(parse(""), vec![]);
        assert_eq!(parse("abc"), vec![Token::Text("abc")]);
        assert_eq!(parse("__"), vec![Token::NewLine]);
        assert_eq!(parse("<abc>"), vec![Token::Var("abc")]);
        assert_eq!(parse("<abc><abc>"), vec![Token::Var("abc"), Token::Var("abc")]);
        assert_eq!(parse("{abc}"), vec![Token::Var("abc")]);
        assert_eq!(parse("{abc}{abc}"), vec![Token::Var("abc"), Token::Var("abc")]);
        assert_eq!(parse("abc__def"), vec![Token::Text("abc"), Token::NewLine, Token::Text("def")]);
        assert_eq!(parse("abc<def>ghi"), vec![Token::Text("abc"), Token::Var("def"), Token::Text("ghi")]);
        assert_eq!(parse("abc{def}ghi"), vec![Token::Text("abc"), Token::Var("def"), Token::Text("ghi")]);
        assert_eq!(parse("abc__def<ghi>{jkl}"), vec![Token::Text("abc"), Token::NewLine, Token::Text("def"), Token::Var("ghi"), Token::Var("jkl")]);
    }
}