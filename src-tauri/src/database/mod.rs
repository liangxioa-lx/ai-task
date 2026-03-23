mod flows;
mod schema;
mod settings;
mod state;
mod tasks;
mod utils;

use serde::Serialize;
use tauri::State;

pub use flows::{
  get_complex_flow_by_task,
  upsert_complex_flow,
  ComplexTaskFlowRecord,
  ComplexTaskNodeRecord,
  UpsertComplexFlowInput,
  UpsertComplexFlowNodeInput
};
pub use settings::{
  delete_setting,
  get_setting,
  list_settings,
  upsert_setting,
  SettingRecord,
  UpsertSettingInput
};
pub use state::DatabaseState;
pub use tasks::{
  create_task,
  delete_task,
  list_tasks,
  update_task_execution,
  CreateTaskInput,
  TaskRecord,
  UpdateTaskExecutionInput
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseHealth {
  pub ready: bool,
  pub db_path: String
}

#[tauri::command]
pub fn init_database(state: State<'_, DatabaseState>) -> Result<DatabaseHealth, String> {
  let _guard = state.acquire_lock()?;
  let _conn = state.open_connection()?;

  Ok(DatabaseHealth {
    ready: true,
    db_path: state.path()
  })
}
