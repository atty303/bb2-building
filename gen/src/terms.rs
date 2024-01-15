use std::sync::OnceLock;
use regex::Regex;
use yaml_rust::YamlLoader;

use data::{LANGUAGES};
use data::term::{Term, TermMap};

pub fn write_terms() {
    let s = std::fs::read_to_string("dump/asset/ExportedProject/Assets/Resources/I2Languages.asset").unwrap();
    let docs = YamlLoader::load_from_str(s.as_str()).unwrap();
    let doc = &docs[0];
    let terms = doc["MonoBehaviour"]["mSource"]["mTerms"].as_vec().unwrap();

    println!("{:?}", terms.len());

    let mut outs = Vec::new();
    for _lang in LANGUAGES {
        outs.push(TermMap::new());
    }

    for term in terms {
        let key = term["Term"].as_str().unwrap();
        let langs = term["Languages"].as_vec().unwrap();
        for (i, _lang) in LANGUAGES.iter().enumerate() {
            let value = langs[2 + i].as_str().unwrap();
            let value = if (value == "$ja") {
                langs[2].as_str().unwrap()
            } else if (value == "$en") {
                langs[3].as_str().unwrap()
            } else {
                value
            };
            outs[i].insert(key.to_string(), Term { value: value.to_string() });
        }
    }

    let re = Regex::new(r"\{\[(.+?)]}").unwrap();
    for (i, lang) in LANGUAGES.iter().enumerate() {
        let out = &outs[i];

        let mut new_out = TermMap::new();
        for (key, value) in out.iter() {
            if let Some(m) = re.captures(&value.value) {
                let key_ref = &m[1].to_string();
                let substitute = &out.get(key_ref);
                if let Some(s) = substitute {
                    new_out.insert(key.clone(), Term { value: s.value.clone() });
                } else {
                    println!("{}: '{}' not found", lang, key_ref);
                    new_out.insert(key.clone(), value.clone());
                }
            } else {
                new_out.insert(key.clone(), value.clone());
            }
        }

        let file_writer = std::io::BufWriter::new(std::fs::File::create(format!("public/i18n/{}/terms.avro", lang)).unwrap());
        TermMap::write(file_writer, &new_out).unwrap();

        {
            let file_writer = std::io::BufWriter::new(std::fs::File::create(format!("dump/{}.csv", lang)).unwrap());
            let mut csv_writer = csv::Writer::from_writer(file_writer);
            for (key, value) in new_out.iter() {
                csv_writer.write_record(&[key, &value.value]).unwrap();
            }
        }
    }
}

static RE: OnceLock<Regex> = OnceLock::new();

use crate::data::term::Node;

pub fn parse<'a>(s: &'a str) -> Vec<Node> {
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
                nodes.push(Node::Var(&s[at + 1..end - 1]));
            } else if span.starts_with("{") {
                nodes.push(Node::Var(&s[at + 1..end - 1]));
            } else if span == "__" {
                nodes.push(Node::NewLine);
            } else {
                nodes.push(Node::Text(span));
            }
            at = end;
        }
    }
    nodes
}

#[cfg(test)]
mod test {
    use terms::parse;
    use super::Node;

    #[test]
    fn test_parse() {
        assert_eq!(parse(""), vec![]);
        assert_eq!(parse("abc"), vec![Node::Text("abc")]);
        assert_eq!(parse("__"), vec![Node::NewLine]);
        assert_eq!(parse("<abc>"), vec![Node::Var("abc")]);
        assert_eq!(parse("<abc><abc>"), vec![Node::Var("abc"), Node::Var("abc")]);
        assert_eq!(parse("{abc}"), vec![Node::Var("abc")]);
        assert_eq!(parse("{abc}{abc}"), vec![Node::Var("abc"), Node::Var("abc")]);
        assert_eq!(parse("abc__def"), vec![Node::Text("abc"), Node::NewLine, Node::Text("def")]);
        assert_eq!(parse("abc<def>ghi"), vec![Node::Text("abc"), Node::Var("def"), Node::Text("ghi")]);
        assert_eq!(parse("abc{def}ghi"), vec![Node::Text("abc"), Node::Var("def"), Node::Text("ghi")]);
        assert_eq!(parse("abc__def<ghi>{jkl}"), vec![Node::Text("abc"), Node::NewLine, Node::Text("def"), Node::Var("ghi"), Node::Var("jkl")]);
    }
}