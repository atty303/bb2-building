use std::collections::HashMap;
use std::io::Write;
use std::marker::PhantomData;

use json::JsonValue;
use prettytable::{Cell, Row};

pub mod act;
pub mod act_node;
pub mod state;
pub mod skill;
pub mod skill_mode;
pub mod sm_act;

pub trait TableParser {
    type Row;

    fn parse_row(p: &EntityParser) -> Self::Row;
}

pub struct EntityParser {
    row_id: String,
    values: HashMap<String, JsonValue>,
}

impl EntityParser {
    pub fn new(json_value: &JsonValue) -> Self {
        let row_id = json_value["Id"].as_str().unwrap().to_owned();

        let mut out = HashMap::new();
        for field in json_value["Values"].members() {
            let name = field["Name"].as_str().unwrap();
            out.insert(name.to_string(), field["Value"].clone());
        }

        Self {
            row_id,
            values: out,
        }
    }

    pub fn row_id(&self) -> String {
        self.row_id.clone()
    }

    pub fn get_str(&self, name: &str) -> String {
        self.values[name].as_str().expect(format!("field {} should be string", name).as_str()).to_string()
    }

    pub fn get_usize(&self, name: &str) -> usize {
        self.get_str(name).parse().expect(format!("field {} should be usize", name).as_str())
    }

    pub fn get_i32(&self, name: &str) -> i32 {
        self.get_str(name).parse().expect(format!("field {} should be i32", name).as_str())
    }

    pub fn get_bool(&self, name: &str) -> bool {
        match self.get_str(name).as_str() {
            "0" => false,
            "1" => true,
            v => panic!("field {} should be bool: {:?}", name, v),
        }
    }
}

pub struct Table<T> {
    meta: JsonValue,
    phantom_data: PhantomData<T>,
}

impl<T: TableParser> Table<T> {
    pub fn new(meta: JsonValue) -> Table<T> {
        Table {
            meta,
            phantom_data: PhantomData,
        }
    }

    pub fn iter(&self) -> Box<dyn Iterator<Item = T::Row> + '_> {
        Box::new(self.meta["Entities"].members().map(|e| {
            let ep = EntityParser::new(e);
            T::parse_row(&ep)
        }))
    }
}

pub struct BGTable<'a> {
    db: &'a JsonValue,
}

impl BGTable<'_> {
    pub fn new(db: &JsonValue) -> BGTable {
        BGTable {
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
