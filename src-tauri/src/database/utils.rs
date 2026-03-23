use std::time::{SystemTime, UNIX_EPOCH};

pub fn now_millis() -> i64 {
  match SystemTime::now().duration_since(UNIX_EPOCH) {
    Ok(duration) => duration.as_millis() as i64,
    Err(_) => 0
  }
}

pub fn now_text() -> String {
  now_millis().to_string()
}

pub fn new_log_id(task_id: &str) -> String {
  let nanos = match SystemTime::now().duration_since(UNIX_EPOCH) {
    Ok(duration) => duration.as_nanos(),
    Err(_) => 0
  };

  format!("log_{task_id}_{nanos}")
}
