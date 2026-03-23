use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};

use rusqlite::Connection;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};

use super::schema::ensure_schema;

pub struct DatabaseState {
  db_path: PathBuf,
  lock: Mutex<()>
}

impl DatabaseState {
  pub fn new(app: &AppHandle) -> Result<Self, String> {
    let app_dir = app
      .path()
      .resolve("ai-task-manager", BaseDirectory::AppLocalData)
      .map_err(|error| format!("failed to resolve app data dir: {error}"))?;

    std::fs::create_dir_all(&app_dir)
      .map_err(|error| format!("failed to create app data dir: {error}"))?;

    let db_path = app_dir.join("ai-task-manager.sqlite");
    let conn = Connection::open(&db_path)
      .map_err(|error| format!("failed to open sqlite database: {error}"))?;

    ensure_schema(&conn)?;

    Ok(Self {
      db_path,
      lock: Mutex::new(())
    })
  }

  pub fn open_connection(&self) -> Result<Connection, String> {
    let conn = Connection::open(&self.db_path)
      .map_err(|error| format!("failed to open sqlite database: {error}"))?;

    ensure_schema(&conn)?;
    Ok(conn)
  }

  pub fn acquire_lock(&self) -> Result<MutexGuard<'_, ()>, String> {
    self
      .lock
      .lock()
      .map_err(|_| "database lock is poisoned".to_string())
  }

  pub fn path(&self) -> String {
    self.db_path.to_string_lossy().to_string()
  }
}
