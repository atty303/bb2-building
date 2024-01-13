extern crate data;
extern crate yaml_rust;

use std::{fs, io};

use yaml_rust::YamlLoader;

use data::{LANGUAGES, Term, TermMap};

fn write_terms() {
    let s = fs::read_to_string("dump/asset/ExportedProject/Assets/Resources/I2Languages.asset").unwrap();
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
            outs[i].insert(key.to_string(), Term { value: value.to_string() });
        }
    }

    for (i, lang) in LANGUAGES.iter().enumerate() {
        let file_writer = io::BufWriter::new(fs::File::create(format!("public/i18n/{}/terms.avro", lang)).unwrap());
        TermMap::write(file_writer, &outs[i]).unwrap();
    }
}

fn main() {
    write_terms();

}