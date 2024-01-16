use std::collections::HashMap;
use std::io::Write;
use std::slice::Iter;

use json::JsonValue;

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

pub struct UnknownTable;
impl TableParser for UnknownTable {
    type Row = ();
    fn parse_row(_: &EntityParser) -> Self::Row {
        ()
    }
}

pub struct Table<T: TableParser> {
    meta: JsonValue,
    fields: Vec<String>,
    rows: Vec<T::Row>,
}

impl<T: TableParser> Table<T> {
    pub fn new(meta: JsonValue) -> Table<T> {
        let mut fields = vec!["_row_id".to_string()];
        for field in meta["Fields"].members() {
            fields.push(field["Name"].as_str().unwrap().to_string());
        }

        let rows = meta["Entities"].members().map(|e| {
            let ep = EntityParser::new(e);
            T::parse_row(&ep)
        }).collect::<Vec<_>>();

        Table {
            meta,
            fields,
            rows,
        }
    }

    pub fn name(&self) -> &str {
        self.meta["Name"].as_str().unwrap()
    }

    pub fn iter(&self) -> Iter<'_, <T as TableParser>::Row> {
        self.rows.iter()
    }

    pub fn fields(&self) -> Iter<'_, String> {
        self.fields.iter()
    }

    pub fn to_csv<W: Write>(&self, writer: W) {
        let mut csv_writer = csv::Writer::from_writer(writer);

        csv_writer.write_record(self.fields()).unwrap();

        for e in self.meta["Entities"].members() {
            let mut row = vec![e["Id"].as_str().unwrap().to_string()];
            for field in e["Values"].members() {
                row.push(field["Value"].as_str().unwrap().to_string());
            }
            csv_writer.write_record(row).unwrap();
        }
    }
}
