use data::state::State;
use table::state::StateTable;
use table::Table;

pub fn process_state(state_table: &Table<StateTable>) {
    let rows = state_table.iter().map(|state_row| {
        State {
            row_id: state_row.row_id.clone(),
            id: state_row.id.clone(),
            format: state_row.format.clone(),
            long_format: state_row.long_format.clone(),
        }
    }).collect::<Vec<_>>();

    let file_writer = std::io::BufWriter::new(std::fs::File::create(format!("public/data/state.avro")).unwrap());
    data::write_avro(file_writer, rows.iter()).unwrap();
}
