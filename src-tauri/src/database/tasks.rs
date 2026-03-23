use rusqlite::{params, Connection, OptionalExtension, Row};
use serde::{Deserialize, Serialize};
use tauri::State;

use super::state::DatabaseState;
use super::utils::{new_log_id, now_text};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskRecord {
  pub task_id: String,
  pub task_name: String,
  pub task_description: String,
  pub task_type: String,
  pub task_complexity_type: String,
  pub schedule_rule: Option<String>,
  pub permission_policy_json: String,
  pub execution_count: i64,
  pub last_executed_at: Option<String>,
  pub execution_status: String,
  pub execution_result: String,
  pub flow_id: Option<String>,
  pub status: String,
  pub created_at: String,
  pub updated_at: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskInput {
  pub task_id: String,
  pub task_name: String,
  pub task_description: Option<String>,
  pub task_type: String,
  pub task_complexity_type: String,
  pub schedule_rule: Option<String>,
  pub permission_policy_json: Option<String>,
  pub status: Option<String>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskExecutionInput {
  pub task_id: String,
  pub execution_status: String,
  pub execution_result: String,
  pub error_message: Option<String>,
  pub executed_at: Option<String>,
  pub increase_count: Option<bool>
}

fn map_task_row(row: &Row<'_>) -> rusqlite::Result<TaskRecord> {
  Ok(TaskRecord {
    task_id: row.get(0)?,
    task_name: row.get(1)?,
    task_description: row.get(2)?,
    task_type: row.get(3)?,
    task_complexity_type: row.get(4)?,
    schedule_rule: row.get(5)?,
    permission_policy_json: row.get(6)?,
    execution_count: row.get(7)?,
    last_executed_at: row.get(8)?,
    execution_status: row.get(9)?,
    execution_result: row.get(10)?,
    flow_id: row.get(11)?,
    status: row.get(12)?,
    created_at: row.get(13)?,
    updated_at: row.get(14)?
  })
}

pub fn find_task(conn: &Connection, task_id: &str) -> Result<Option<TaskRecord>, String> {
  conn
    .query_row(
      "
      SELECT
        task_id,
        task_name,
        task_description,
        task_type,
        task_complexity_type,
        schedule_rule,
        permission_policy_json,
        execution_count,
        last_executed_at,
        execution_status,
        execution_result,
        flow_id,
        status,
        created_at,
        updated_at
      FROM tasks
      WHERE task_id = ?1
      ",
      params![task_id],
      map_task_row
    )
    .optional()
    .map_err(|error| format!("failed to query task: {error}"))
}

#[tauri::command]
pub fn list_tasks(state: State<'_, DatabaseState>) -> Result<Vec<TaskRecord>, String> {
  let _guard = state.acquire_lock()?;
  let conn = state.open_connection()?;

  let mut stmt = conn
    .prepare(
      "
      SELECT
        task_id,
        task_name,
        task_description,
        task_type,
        task_complexity_type,
        schedule_rule,
        permission_policy_json,
        execution_count,
        last_executed_at,
        execution_status,
        execution_result,
        flow_id,
        status,
        created_at,
        updated_at
      FROM tasks
      ORDER BY updated_at DESC
      "
    )
    .map_err(|error| format!("failed to prepare task query: {error}"))?;

  stmt
    .query_map([], map_task_row)
    .map_err(|error| format!("failed to query tasks: {error}"))?
    .collect::<rusqlite::Result<Vec<TaskRecord>>>()
    .map_err(|error| format!("failed to map tasks: {error}"))
}

#[tauri::command]
pub fn create_task(state: State<'_, DatabaseState>, input: CreateTaskInput) -> Result<TaskRecord, String> {
  if input.task_id.trim().is_empty() {
    return Err("taskId is required".to_string());
  }

  if input.task_name.trim().is_empty() {
    return Err("taskName is required".to_string());
  }

  let _guard = state.acquire_lock()?;
  let conn = state.open_connection()?;
  let now = now_text();

  conn
    .execute(
      "
      INSERT INTO tasks (
        task_id,
        task_name,
        task_description,
        task_type,
        task_complexity_type,
        schedule_rule,
        permission_policy_json,
        execution_count,
        last_executed_at,
        execution_status,
        execution_result,
        flow_id,
        status,
        created_at,
        updated_at
      ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0, NULL, 'idle', 'unknown', NULL, ?8, ?9, ?9)
      ",
      params![
        input.task_id,
        input.task_name,
        input.task_description.unwrap_or_default(),
        input.task_type,
        input.task_complexity_type,
        input.schedule_rule,
        input.permission_policy_json.unwrap_or_else(|| "{}".to_string()),
        input.status.unwrap_or_else(|| "enabled".to_string()),
        now
      ]
    )
    .map_err(|error| format!("failed to create task: {error}"))?;

  find_task(&conn, &input.task_id)?.ok_or_else(|| "task was not found after insert".to_string())
}

#[tauri::command]
pub fn delete_task(state: State<'_, DatabaseState>, task_id: String) -> Result<(), String> {
  let _guard = state.acquire_lock()?;
  let conn = state.open_connection()?;

  conn
    .execute("DELETE FROM tasks WHERE task_id = ?1", params![task_id])
    .map_err(|error| format!("failed to delete task: {error}"))?;

  Ok(())
}

#[tauri::command]
pub fn update_task_execution(
  state: State<'_, DatabaseState>,
  input: UpdateTaskExecutionInput
) -> Result<TaskRecord, String> {
  let _guard = state.acquire_lock()?;
  let conn = state.open_connection()?;

  let now = now_text();
  let executed_at = input.executed_at.unwrap_or_else(|| now.clone());
  let increase_count = input.increase_count.unwrap_or(false);

  conn
    .execute(
      "
      UPDATE tasks
      SET
        execution_status = ?1,
        execution_result = ?2,
        last_executed_at = ?3,
        execution_count = execution_count + ?4,
        updated_at = ?5
      WHERE task_id = ?6
      ",
      params![
        input.execution_status,
        input.execution_result,
        executed_at,
        if increase_count { 1 } else { 0 },
        now,
        input.task_id
      ]
    )
    .map_err(|error| format!("failed to update task execution: {error}"))?;

  conn
    .execute(
      "
      INSERT INTO task_execution_logs (
        log_id,
        task_id,
        trigger_type,
        execution_status,
        execution_result,
        error_message,
        executed_at,
        created_at
      ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
      ",
      params![
        new_log_id(&input.task_id),
        input.task_id,
        if increase_count { "manual" } else { "state_sync" },
        input.execution_status,
        input.execution_result,
        input.error_message,
        executed_at,
        now
      ]
    )
    .map_err(|error| format!("failed to write task execution log: {error}"))?;

  find_task(&conn, &input.task_id)?.ok_or_else(|| "task was not found after update".to_string())
}
