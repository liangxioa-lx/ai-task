use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use tauri::State;

use super::state::DatabaseState;
use super::tasks::find_task;
use super::utils::now_text;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComplexTaskFlowRecord {
  pub flow_id: String,
  pub parent_task_id: String,
  pub flow_name: Option<String>,
  pub flow_description: Option<String>,
  pub version: i64,
  pub status: String,
  pub created_at: String,
  pub updated_at: String,
  pub nodes: Vec<ComplexTaskNodeRecord>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComplexTaskNodeRecord {
  pub node_id: String,
  pub flow_id: String,
  pub ref_task_id: String,
  pub node_type: String,
  pub position: i64,
  pub condition_expr: Option<String>,
  pub on_success_next: Option<String>,
  pub on_failure_next: Option<String>,
  pub created_at: String,
  pub updated_at: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpsertComplexFlowInput {
  pub flow_id: String,
  pub parent_task_id: String,
  pub flow_name: Option<String>,
  pub flow_description: Option<String>,
  pub version: Option<i64>,
  pub status: Option<String>,
  pub nodes: Vec<UpsertComplexFlowNodeInput>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpsertComplexFlowNodeInput {
  pub node_id: String,
  pub ref_task_id: String,
  pub node_type: String,
  pub position: i64,
  pub condition_expr: Option<String>,
  pub on_success_next: Option<String>,
  pub on_failure_next: Option<String>
}

pub fn find_flow_by_task(conn: &Connection, parent_task_id: &str) -> Result<Option<ComplexTaskFlowRecord>, String> {
  let flow = conn
    .query_row(
      "
      SELECT
        flow_id,
        parent_task_id,
        flow_name,
        flow_description,
        version,
        status,
        created_at,
        updated_at
      FROM complex_task_flows
      WHERE parent_task_id = ?1
      LIMIT 1
      ",
      params![parent_task_id],
      |row| {
        Ok(ComplexTaskFlowRecord {
          flow_id: row.get(0)?,
          parent_task_id: row.get(1)?,
          flow_name: row.get(2)?,
          flow_description: row.get(3)?,
          version: row.get(4)?,
          status: row.get(5)?,
          created_at: row.get(6)?,
          updated_at: row.get(7)?,
          nodes: Vec::new()
        })
      }
    )
    .optional()
    .map_err(|error| format!("failed to query flow: {error}"))?;

  let Some(mut flow_record) = flow else {
    return Ok(None);
  };

  let mut stmt = conn
    .prepare(
      "
      SELECT
        node_id,
        flow_id,
        ref_task_id,
        node_type,
        position,
        condition_expr,
        on_success_next,
        on_failure_next,
        created_at,
        updated_at
      FROM complex_task_flow_nodes
      WHERE flow_id = ?1
      ORDER BY position ASC
      "
    )
    .map_err(|error| format!("failed to prepare node query: {error}"))?;

  let nodes = stmt
    .query_map(params![&flow_record.flow_id], |row| {
      Ok(ComplexTaskNodeRecord {
        node_id: row.get(0)?,
        flow_id: row.get(1)?,
        ref_task_id: row.get(2)?,
        node_type: row.get(3)?,
        position: row.get(4)?,
        condition_expr: row.get(5)?,
        on_success_next: row.get(6)?,
        on_failure_next: row.get(7)?,
        created_at: row.get(8)?,
        updated_at: row.get(9)?
      })
    })
    .map_err(|error| format!("failed to query flow nodes: {error}"))?
    .collect::<rusqlite::Result<Vec<ComplexTaskNodeRecord>>>()
    .map_err(|error| format!("failed to map flow nodes: {error}"))?;

  flow_record.nodes = nodes;
  Ok(Some(flow_record))
}

#[tauri::command]
pub fn upsert_complex_flow(
  state: State<'_, DatabaseState>,
  input: UpsertComplexFlowInput
) -> Result<ComplexTaskFlowRecord, String> {
  if input.flow_id.trim().is_empty() {
    return Err("flowId is required".to_string());
  }

  if input.parent_task_id.trim().is_empty() {
    return Err("parentTaskId is required".to_string());
  }

  let _guard = state.acquire_lock()?;
  let mut conn = state.open_connection()?;

  if find_task(&conn, &input.parent_task_id)?.is_none() {
    return Err("parent task not found".to_string());
  }

  let tx = conn
    .transaction()
    .map_err(|error| format!("failed to start transaction: {error}"))?;

  let now = now_text();
  let version = input.version.unwrap_or(1);

  tx
    .execute(
      "
      INSERT INTO complex_task_flows (
        flow_id,
        parent_task_id,
        flow_name,
        flow_description,
        version,
        status,
        created_at,
        updated_at
      ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7)
      ON CONFLICT(flow_id)
      DO UPDATE SET
        parent_task_id = excluded.parent_task_id,
        flow_name = excluded.flow_name,
        flow_description = excluded.flow_description,
        version = excluded.version,
        status = excluded.status,
        updated_at = excluded.updated_at
      ",
      params![
        input.flow_id,
        input.parent_task_id,
        input.flow_name,
        input.flow_description,
        version,
        input.status.unwrap_or_else(|| "draft".to_string()),
        now
      ]
    )
    .map_err(|error| format!("failed to upsert flow: {error}"))?;

  tx
    .execute(
      "DELETE FROM complex_task_flow_nodes WHERE flow_id = ?1",
      params![input.flow_id]
    )
    .map_err(|error| format!("failed to clear old flow nodes: {error}"))?;

  for node in &input.nodes {
    tx
      .execute(
        "
        INSERT INTO complex_task_flow_nodes (
          node_id,
          flow_id,
          ref_task_id,
          node_type,
          position,
          condition_expr,
          on_success_next,
          on_failure_next,
          created_at,
          updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9)
        ON CONFLICT(node_id)
        DO UPDATE SET
          flow_id = excluded.flow_id,
          ref_task_id = excluded.ref_task_id,
          node_type = excluded.node_type,
          position = excluded.position,
          condition_expr = excluded.condition_expr,
          on_success_next = excluded.on_success_next,
          on_failure_next = excluded.on_failure_next,
          updated_at = excluded.updated_at
        ",
        params![
          node.node_id,
          input.flow_id,
          node.ref_task_id,
          node.node_type,
          node.position,
          node.condition_expr,
          node.on_success_next,
          node.on_failure_next,
          now
        ]
      )
      .map_err(|error| format!("failed to upsert flow node: {error}"))?;
  }

  tx
    .execute(
      "
      UPDATE tasks
      SET task_complexity_type = 'complex', flow_id = ?1, updated_at = ?2
      WHERE task_id = ?3
      ",
      params![input.flow_id, now, input.parent_task_id]
    )
    .map_err(|error| format!("failed to link flow to task: {error}"))?;

  tx
    .commit()
    .map_err(|error| format!("failed to commit flow transaction: {error}"))?;

  find_flow_by_task(&conn, &input.parent_task_id)?
    .ok_or_else(|| "flow was not found after upsert".to_string())
}

#[tauri::command]
pub fn get_complex_flow_by_task(
  state: State<'_, DatabaseState>,
  task_id: String
) -> Result<Option<ComplexTaskFlowRecord>, String> {
  let _guard = state.acquire_lock()?;
  let conn = state.open_connection()?;

  find_flow_by_task(&conn, &task_id)
}
