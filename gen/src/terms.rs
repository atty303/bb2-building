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
    }
}
