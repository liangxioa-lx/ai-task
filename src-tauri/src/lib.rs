mod database;

use database::DatabaseState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      let state = DatabaseState::new(app.handle()).map_err(|error| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("database init failed: {error}"))
      })?;

      app.manage(state);
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      database::init_database,
      database::list_tasks,
      database::create_task,
      database::delete_task,
      database::update_task_execution,
      database::upsert_complex_flow,
      database::get_complex_flow_by_task,
      database::list_settings,
      database::get_setting,
      database::upsert_setting,
      database::delete_setting
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
