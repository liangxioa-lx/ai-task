use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use tauri::State;

use super::state::DatabaseState;
use super::utils::now_text;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingRecord {
  pub setting_key: String,
  pub setting_value: String,
  pub value_type: String,
  pub created_at: String,
  pub updated_at: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpsertSettingInput {
  pub setting_key: String,
  pub setting_value: String,
  pub value_type: Option<String>
}

fn find_setting(conn: &rusqlite::Connection, setting_key: &str) -> Result<Option<SettingRecord>, String> {
  conn
    .query_row(
      "
      SELECT setting_key, setting_value, value_type, created_at, updated_at
      FROM settings
      WHERE setting_key = ?1
      ",
      params![setting_key],
      |row| {
        Ok(SettingRecord {
          setting_key: row.get(0)?,
          setting_value: row.get(1)?,
          value_type: row.get(2)?,
          created_at: row.get(3)?,
          updated_at: row.get(4)?
        })
      }
    )
    .optional()
    .map_err(|error| format!("failed to query setting: {error}"))
}

#[tauri::command]
pub fn list_settings(state: State<'_, DatabaseState>) -> Result<Vec<SettingRecord>, String> {
  let _guard = state.acquire_lock()?;
  let conn = state.open_connection()?;

  let mut stmt = conn
    .prepare(
      "
      SELECT setting_key, setting_value, value_type, created_at, updated_at
      FROM settings
      ORDER BY updated_at DESC
      "
    )
    .map_err(|error| format!("failed to prepare settings query: {error}"))?;

  stmt
    .query_map([], |row| {
      Ok(SettingRecord {
        setting_key: row.get(0)?,
        setting_value: row.get(1)?,
        value_type: row.get(2)?,
        created_at: row.get(3)?,
        updated_at: row.get(4)?
      })
    })
    .map_err(|error| format!("failed to query settings: {error}"))?
    .collect::<rusqlite::Result<Vec<SettingRecord>>>()
    .map_err(|error| format!("failed to map settings: {error}"))
}

#[tauri::command]
pub fn get_setting(state: State<'_, DatabaseState>, setting_key: String) -> Result<Option<SettingRecord>, String> {
  let _guard = state.acquire_lock()?;
  let conn = state.open_connection()?;

  find_setting(&conn, &setting_key)
}

#[tauri::command]
pub fn upsert_setting(state: State<'_, DatabaseState>, input: UpsertSettingInput) -> Result<SettingRecord, String> {
  if input.setting_key.trim().is_empty() {
    return Err("settingKey is required".to_string());
  }

  let _guard = state.acquire_lock()?;
  let conn = state.open_connection()?;
  let now = now_text();

  conn
    .execute(
      "
      INSERT INTO settings (setting_key, setting_value, value_type, created_at, updated_at)
      VALUES (?1, ?2, ?3, ?4, ?4)
      ON CONFLICT(setting_key)
      DO UPDATE SET
        setting_value = excluded.setting_value,
        value_type = excluded.value_type,
        updated_at = excluded.updated_at
      ",
      params![
        input.setting_key,
        input.setting_value,
        input.value_type.unwrap_or_else(|| "json".to_string()),
        now
      ]
    )
    .map_err(|error| format!("failed to upsert setting: {error}"))?;

  find_setting(&conn, &input.setting_key)?.ok_or_else(|| "setting was not found after upsert".to_string())
}

#[tauri::command]
pub fn delete_setting(state: State<'_, DatabaseState>, setting_key: String) -> Result<(), String> {
  let _guard = state.acquire_lock()?;
  let conn = state.open_connection()?;

  conn
    .execute("DELETE FROM settings WHERE setting_key = ?1", params![setting_key])
    .map_err(|error| format!("failed to delete setting: {error}"))?;

  Ok(())
}
