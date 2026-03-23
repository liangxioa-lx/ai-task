use rusqlite::Connection;

pub fn ensure_schema(conn: &Connection) -> Result<(), String> {
  conn
    .execute_batch(
      "
      PRAGMA foreign_keys = ON;

      CREATE TABLE IF NOT EXISTS tasks (
        task_id TEXT PRIMARY KEY,
        task_name TEXT NOT NULL,
        task_description TEXT NOT NULL DEFAULT '',
        task_type TEXT NOT NULL,
        task_complexity_type TEXT NOT NULL DEFAULT 'simple',
        schedule_rule TEXT,
        permission_policy_json TEXT NOT NULL DEFAULT '{}',
        execution_count INTEGER NOT NULL DEFAULT 0,
        last_executed_at TEXT,
        execution_status TEXT NOT NULL DEFAULT 'idle',
        execution_result TEXT NOT NULL DEFAULT 'unknown',
        flow_id TEXT,
        status TEXT NOT NULL DEFAULT 'enabled',
        created_at TEXT NOT NULL,
        updated_at TEXT NOT NULL
      );

      CREATE TABLE IF NOT EXISTS complex_task_flows (
        flow_id TEXT PRIMARY KEY,
        parent_task_id TEXT NOT NULL,
        flow_name TEXT,
        flow_description TEXT,
        version INTEGER NOT NULL DEFAULT 1,
        status TEXT NOT NULL DEFAULT 'draft',
        created_at TEXT NOT NULL,
        updated_at TEXT NOT NULL,
        FOREIGN KEY(parent_task_id) REFERENCES tasks(task_id) ON DELETE CASCADE
      );

      CREATE TABLE IF NOT EXISTS complex_task_flow_nodes (
        node_id TEXT PRIMARY KEY,
        flow_id TEXT NOT NULL,
        ref_task_id TEXT NOT NULL,
        node_type TEXT NOT NULL,
        position INTEGER NOT NULL DEFAULT 0,
        condition_expr TEXT,
        on_success_next TEXT,
        on_failure_next TEXT,
        created_at TEXT NOT NULL,
        updated_at TEXT NOT NULL,
        FOREIGN KEY(flow_id) REFERENCES complex_task_flows(flow_id) ON DELETE CASCADE
      );

      CREATE TABLE IF NOT EXISTS task_execution_logs (
        log_id TEXT PRIMARY KEY,
        task_id TEXT NOT NULL,
        trigger_type TEXT NOT NULL,
        execution_status TEXT NOT NULL,
        execution_result TEXT NOT NULL,
        error_message TEXT,
        executed_at TEXT NOT NULL,
        created_at TEXT NOT NULL,
        FOREIGN KEY(task_id) REFERENCES tasks(task_id) ON DELETE CASCADE
      );

      CREATE TABLE IF NOT EXISTS settings (
        setting_key TEXT PRIMARY KEY,
        setting_value TEXT NOT NULL,
        value_type TEXT NOT NULL DEFAULT 'json',
        created_at TEXT NOT NULL,
        updated_at TEXT NOT NULL
      );

      CREATE INDEX IF NOT EXISTS idx_tasks_updated_at ON tasks(updated_at DESC);
      CREATE INDEX IF NOT EXISTS idx_task_logs_task_time ON task_execution_logs(task_id, executed_at DESC);
      CREATE INDEX IF NOT EXISTS idx_flow_nodes_flow_position ON complex_task_flow_nodes(flow_id, position ASC);
      CREATE INDEX IF NOT EXISTS idx_settings_updated_at ON settings(updated_at DESC);
      "
    )
    .map_err(|error| format!("failed to ensure sqlite schema: {error}"))
}
