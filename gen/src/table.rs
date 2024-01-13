use std::collections::HashMap;
use std::io::Write;

use json::JsonValue;
use prettytable::{Cell, Row};

pub struct Table<'a> {
    db: &'a JsonValue,
}

impl Table<'_> {
    pub fn new(db: &JsonValue) -> Table {
        Table {
            db,
        }
    }

    pub fn id(&self) -> &str {
        self.db["Id"].as_str().unwrap()
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

    pub fn entities(&self) -> Vec<HashMap<String, JsonValue>> {
        let mut out = Vec::new();

        for entity in self.db["Entities"].members() {
            let mut entity_map = HashMap::new();

            entity_map.insert("_row_id".to_string(), entity["Id"].clone());

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

    pub fn to_csv<W: Write>(&self, writer: W) {
        let mut table = prettytable::Table::new();
        table.set_format(*prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

        let fs = self.fields();
        let mut title_rows = Row::new(vec![Cell::new("row_id")]);
        title_rows.extend(fs.clone().into_iter().map(|f| Cell::new(f.as_str())).collect::<Vec<_>>());
        table.set_titles(title_rows);

        let es = self.entities();
        for e in es.iter() {
            let mut row = Row::new(vec![Cell::new(format!("{}", e.get("_row_id").unwrap()).as_str())]);
            row.extend(fs.clone().into_iter().map(|f| Cell::new(format!("{}", e.get(f.as_str()).unwrap()).as_str())).collect::<Vec<_>>());
            table.add_row(row);
        }

        table.to_csv(writer).unwrap();
    }
}
