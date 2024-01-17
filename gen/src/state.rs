use data::state::{State, StateRepository};
use table::state::StateTable;
use table::Table;

pub fn state_repository_from_dump(state_table: &Table<StateTable>) -> StateRepository {
    let rows = state_table
        .iter()
        .map(|state_row| State {
            row_id: state_row.row_id.clone(),
            id: state_row.id.clone(),
            name: state_row.name.clone(),
            format: state_row.format.clone(),
            long_format: state_row.long_format.clone(),
        })
        .collect::<Vec<_>>();

    StateRepository::from_vec(rows)
}
