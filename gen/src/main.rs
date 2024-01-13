mod terms;

extern crate data;
extern crate yaml_rust;
extern crate json;
extern crate prettytable;

use std::collections::HashMap;
use std::io::Write;
use json::JsonValue;
use prettytable::{Cell, Row};

fn main() {
    terms::write_terms();

    let s = std::fs::read_to_string("dump/db.json").unwrap();
    let db_json = json::parse(s.as_str()).unwrap();

    for meta in db_json["Metas"].members() {
        let meta_name = meta["Name"].as_str().unwrap();
        let table = Table::new(meta);
        table.to_csv(std::io::BufWriter::new(std::fs::File::create(format!("dump/table/{}.csv", meta_name)).unwrap()));
    }

    for meta in db_json["Metas"].members() {
        let meta_name = meta["Name"].as_str().unwrap();
        process_table(meta_name, meta);
    }

}

fn process_table(name: &str, meta: &JsonValue) {
    let table = Table::new(meta);
    match name {
        _ => {
            println!();
            println!("unknown table: {:?}", name);
            //table.print_entities();
        },
    }
}

struct Table<'a> {
    db: &'a JsonValue,
}

impl Table<'_> {
    fn new(db: &JsonValue) -> Table {
        Table {
            db,
        }
    }

    fn verify(&self) {
        assert_eq!(self.db["Keys"].len(), 1, "table should have one key");
        assert_eq!(self.db["Keys"][0]["Name"].as_str().unwrap(), "key_ID", "table should have one key named 'id'");
        assert_eq!(self.db["Keys"][0]["Unique"].as_bool().unwrap(), true, "table should have one key named 'id' and it should be unique");
    }

    fn fields(&self) -> Vec<String> {
        let mut out = Vec::new();

        for field in self.db["Fields"].members() {
            out.push(field["Name"].as_str().unwrap().to_string());
        }

        out
    }

    fn entities(&self) -> Vec<HashMap<String, JsonValue>> {
        let mut out = Vec::new();

        for entity in self.db["Entities"].members() {
            let mut entity_map = HashMap::new();

            for field in entity["Values"].members() {
                let field_name = field["Name"].as_str().unwrap();
                let field_value = field["Value"].clone();

                entity_map.insert(field_name.to_string(), field_value);
            }

            out.push(entity_map);
        }

        out
    }

    fn print_fields(&self) {
        print!("fields: ");
        for field in self.db["Fields"].members() {
            print!("{}, ", field["Name"].as_str().unwrap());
        }
        println!();

    }

    fn to_csv<W: Write>(&self, writer: W) {
        let mut table = prettytable::Table::new();
        table.set_format(*prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

        let fs = self.fields();
        table.set_titles(Row::new(fs.iter().map(|f| Cell::new(f)).collect()));

        let es = self.entities();
        for e in es.iter() {
            table.add_row(Row::new(fs.iter().map(|f| Cell::new(format!("{}", e.get(f).unwrap()).as_str())).collect()));
        }

        table.to_csv(writer).unwrap();
    }
}
