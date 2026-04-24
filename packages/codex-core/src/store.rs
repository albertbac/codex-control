use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::Write;

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::{CodexEvent, CodexSession, EventName, SessionStatus};
use crate::paths::ensure_data_dirs;
use crate::repo::discover_repo_context;

const MIGRATIONS: &[(i64, &str)] = &[
  (
    1,
    "CREATE TABLE IF NOT EXISTS schema_migrations (version INTEGER PRIMARY KEY, applied_at TEXT NOT NULL);",
  ),
  (
    2,
    "CREATE TABLE IF NOT EXISTS events (
      id TEXT PRIMARY KEY,
      session_id TEXT NOT NULL,
      turn_id TEXT,
      event_name TEXT NOT NULL,
      cwd TEXT NOT NULL,
      model TEXT,
      payload_json TEXT NOT NULL,
      created_at TEXT NOT NULL
    );",
  ),
  (
    3,
    "CREATE TABLE IF NOT EXISTS sessions (
      id TEXT PRIMARY KEY,
      cwd TEXT NOT NULL,
      repo_root TEXT,
      repo_name TEXT,
      branch TEXT,
      model TEXT,
      transcript_path TEXT,
      status TEXT NOT NULL,
      last_prompt TEXT,
      last_command TEXT,
      last_assistant_message TEXT,
      started_at TEXT NOT NULL,
      updated_at TEXT NOT NULL
    );",
  ),
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum StoreMode {
    Sqlite,
    Spool,
}

impl StoreMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            StoreMode::Sqlite => "sqlite",
            StoreMode::Spool => "spool",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct StoreWriteOutcome {
    pub mode: String,
    pub database_path: String,
    pub spool_path: String,
}

pub struct LocalStore {
    paths: crate::models::DataPaths,
    mode: StoreMode,
}

impl LocalStore {
    pub fn open() -> Result<Self> {
        let paths = ensure_data_dirs()?;
        match Connection::open(&paths.database_path) {
            Ok(connection) => {
                apply_migrations(&connection)?;
                Ok(Self {
                    paths,
                    mode: StoreMode::Sqlite,
                })
            }
            Err(_) => Ok(Self {
                paths,
                mode: StoreMode::Spool,
            }),
        }
    }

    pub fn mode(&self) -> StoreMode {
        self.mode.clone()
    }

    pub fn paths(&self) -> &crate::models::DataPaths {
        &self.paths
    }

    pub fn persist_event(&self, event: &CodexEvent) -> Result<StoreWriteOutcome> {
        match self.try_persist_sqlite(event) {
            Ok(()) => Ok(self.outcome(StoreMode::Sqlite)),
            Err(_) => {
                self.append_spool(event)?;
                Ok(self.outcome(StoreMode::Spool))
            }
        }
    }

    pub fn list_sessions(&self) -> Result<Vec<CodexSession>> {
        match self.try_list_sessions_sqlite() {
            Ok(sessions) => Ok(sessions),
            Err(_) => self.list_sessions_from_spool(),
        }
    }

    pub fn list_events(&self, session_id: &str) -> Result<Vec<CodexEvent>> {
        match self.try_list_events_sqlite(session_id) {
            Ok(events) => Ok(events),
            Err(_) => self.list_events_from_spool(session_id),
        }
    }

    pub fn latest_event_at(&self) -> Result<Option<String>> {
        match self.try_latest_event_at_sqlite() {
            Ok(latest) => Ok(latest),
            Err(_) => Ok(self
                .list_events_from_spool("__all__")?
                .into_iter()
                .map(|event| event.created_at)
                .max()),
        }
    }

    pub fn clear_local_data(&self) -> Result<()> {
        let _ = fs::remove_file(&self.paths.database_path);
        let _ = fs::remove_file(&self.paths.spool_path);
        ensure_data_dirs()?;
        Ok(())
    }

    fn outcome(&self, mode: StoreMode) -> StoreWriteOutcome {
        StoreWriteOutcome {
            mode: mode.as_str().to_string(),
            database_path: self.paths.database_path.clone(),
            spool_path: self.paths.spool_path.clone(),
        }
    }

    fn try_persist_sqlite(&self, event: &CodexEvent) -> Result<()> {
        let mut connection = Connection::open(&self.paths.database_path)?;
        apply_migrations(&connection)?;

        let tx = connection.transaction()?;
        tx.execute(
      "INSERT OR REPLACE INTO events (id, session_id, turn_id, event_name, cwd, model, payload_json, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
      params![
        event.id,
        event.session_id,
        event.turn_id,
        event_name_to_string(&event.event_name),
        event.cwd,
        event.model,
        serde_json::to_string(&event.payload)?,
        event.created_at,
      ],
    )?;

        let existing = select_session(&tx, &event.session_id)?;
        let reduced = reduce_session(existing, event);

        tx.execute(
      "INSERT INTO sessions (id, cwd, repo_root, repo_name, branch, model, transcript_path, status, last_prompt, last_command, last_assistant_message, started_at, updated_at)
      VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
      ON CONFLICT(id) DO UPDATE SET
        cwd = excluded.cwd,
        repo_root = excluded.repo_root,
        repo_name = excluded.repo_name,
        branch = excluded.branch,
        model = excluded.model,
        transcript_path = excluded.transcript_path,
        status = excluded.status,
        last_prompt = excluded.last_prompt,
        last_command = excluded.last_command,
        last_assistant_message = excluded.last_assistant_message,
        started_at = excluded.started_at,
        updated_at = excluded.updated_at",
      params![
        reduced.id,
        reduced.cwd,
        reduced.repo_root,
        reduced.repo_name,
        reduced.branch,
        reduced.model,
        reduced.transcript_path,
        status_to_string(&reduced.status),
        reduced.last_prompt,
        reduced.last_command,
        reduced.last_assistant_message,
        reduced.started_at,
        reduced.updated_at,
      ],
    )?;

        tx.commit()?;
        Ok(())
    }

    fn append_spool(&self, event: &CodexEvent) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.paths.spool_path)
            .context("unable to open spool file")?;
        let json = serde_json::to_string(event)?;
        writeln!(file, "{json}")?;
        Ok(())
    }

    fn try_list_sessions_sqlite(&self) -> Result<Vec<CodexSession>> {
        let connection = Connection::open(&self.paths.database_path)?;
        apply_migrations(&connection)?;
        let mut stmt = connection.prepare(
      "SELECT id, cwd, repo_root, repo_name, branch, model, transcript_path, status, last_prompt, last_command, last_assistant_message, started_at, updated_at FROM sessions ORDER BY updated_at DESC",
    )?;
        let rows = stmt.query_map([], |row| {
            Ok(CodexSession {
                id: row.get(0)?,
                cwd: row.get(1)?,
                repo_root: row.get(2)?,
                repo_name: row.get(3)?,
                branch: row.get(4)?,
                model: row.get(5)?,
                transcript_path: row.get(6)?,
                status: status_from_string(row.get::<_, String>(7)?.as_str()),
                last_prompt: row.get(8)?,
                last_command: row.get(9)?,
                last_assistant_message: row.get(10)?,
                started_at: row.get(11)?,
                updated_at: row.get(12)?,
            })
        })?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    fn try_list_events_sqlite(&self, session_id: &str) -> Result<Vec<CodexEvent>> {
        let connection = Connection::open(&self.paths.database_path)?;
        apply_migrations(&connection)?;
        let mut stmt = connection.prepare(
      "SELECT id, session_id, turn_id, event_name, cwd, model, payload_json, created_at FROM events WHERE (?1 = '__all__' OR session_id = ?1) ORDER BY created_at ASC",
    )?;
        let rows = stmt.query_map([session_id], |row| {
            let payload_text: String = row.get(6)?;
            let payload: Value = serde_json::from_str(&payload_text).unwrap_or(Value::Null);
            Ok(CodexEvent {
                id: row.get(0)?,
                session_id: row.get(1)?,
                turn_id: row.get(2)?,
                event_name: event_name_from_string(row.get::<_, String>(3)?.as_str()),
                cwd: row.get(4)?,
                model: row.get(5)?,
                payload,
                created_at: row.get(7)?,
            })
        })?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    fn try_latest_event_at_sqlite(&self) -> Result<Option<String>> {
        let connection = Connection::open(&self.paths.database_path)?;
        apply_migrations(&connection)?;
        connection
            .query_row(
                "SELECT created_at FROM events ORDER BY created_at DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .optional()
            .map_err(Into::into)
    }

    fn list_sessions_from_spool(&self) -> Result<Vec<CodexSession>> {
        let mut sessions = BTreeMap::<String, CodexSession>::new();
        for event in self.list_events_from_spool("__all__")? {
            let existing = sessions.remove(&event.session_id);
            let reduced = reduce_session(existing, &event);
            sessions.insert(reduced.id.clone(), reduced);
        }
        Ok(sessions.into_values().rev().collect())
    }

    fn list_events_from_spool(&self, session_id: &str) -> Result<Vec<CodexEvent>> {
        let Ok(contents) = fs::read_to_string(&self.paths.spool_path) else {
            return Ok(Vec::new());
        };

        let mut events = Vec::new();
        for line in contents.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let event =
                serde_json::from_str::<CodexEvent>(line).context("invalid event in spool file")?;
            if session_id == "__all__" || event.session_id == session_id {
                events.push(event);
            }
        }
        events.sort_by(|left, right| left.created_at.cmp(&right.created_at));
        Ok(events)
    }
}

fn apply_migrations(connection: &Connection) -> Result<()> {
    connection.execute_batch(MIGRATIONS[0].1)?;
    for (version, statement) in MIGRATIONS.iter().skip(1) {
        let applied: Option<i64> = connection
            .query_row(
                "SELECT version FROM schema_migrations WHERE version = ?1",
                [version],
                |row| row.get(0),
            )
            .optional()?;
        if applied.is_none() {
            connection.execute_batch(statement)?;
            connection.execute(
                "INSERT INTO schema_migrations (version, applied_at) VALUES (?1, ?2)",
                params![version, Utc::now().to_rfc3339()],
            )?;
        }
    }
    Ok(())
}

fn select_session(connection: &Connection, session_id: &str) -> Result<Option<CodexSession>> {
    connection
    .query_row(
      "SELECT id, cwd, repo_root, repo_name, branch, model, transcript_path, status, last_prompt, last_command, last_assistant_message, started_at, updated_at FROM sessions WHERE id = ?1",
      [session_id],
      |row| {
        Ok(CodexSession {
          id: row.get(0)?,
          cwd: row.get(1)?,
          repo_root: row.get(2)?,
          repo_name: row.get(3)?,
          branch: row.get(4)?,
          model: row.get(5)?,
          transcript_path: row.get(6)?,
          status: status_from_string(row.get::<_, String>(7)?.as_str()),
          last_prompt: row.get(8)?,
          last_command: row.get(9)?,
          last_assistant_message: row.get(10)?,
          started_at: row.get(11)?,
          updated_at: row.get(12)?,
        })
      },
    )
    .optional()
    .map_err(Into::into)
}

fn reduce_session(existing: Option<CodexSession>, event: &CodexEvent) -> CodexSession {
    let repo_context = discover_repo_context(&event.cwd);
    let transcript_path = extract_string(&event.payload, "transcript_path");
    let prompt = extract_string(&event.payload, "prompt");
    let command = extract_command(&event.payload);
    let assistant_message = extract_string(&event.payload, "last_assistant_message");

    let mut session = existing.unwrap_or(CodexSession {
        id: event.session_id.clone(),
        cwd: event.cwd.clone(),
        repo_root: repo_context.repo_root.clone(),
        repo_name: repo_context.repo_name.clone(),
        branch: repo_context.branch.clone(),
        model: event.model.clone(),
        transcript_path: transcript_path.clone(),
        status: SessionStatus::Unknown,
        last_prompt: None,
        last_command: None,
        last_assistant_message: None,
        started_at: event.created_at.clone(),
        updated_at: event.created_at.clone(),
    });

    session.cwd = event.cwd.clone();
    session.repo_root = session.repo_root.or(repo_context.repo_root);
    session.repo_name = session.repo_name.or(repo_context.repo_name);
    session.branch = session.branch.or(repo_context.branch);
    session.model = event.model.clone().or(session.model);
    session.transcript_path = transcript_path.or(session.transcript_path);
    session.updated_at = event.created_at.clone();

    if matches!(event.event_name, EventName::UserPromptSubmit) {
        session.last_prompt = prompt.or(session.last_prompt);
    }

    if matches!(
        event.event_name,
        EventName::PreToolUse | EventName::PermissionRequest | EventName::PostToolUse
    ) {
        session.last_command = command.or(session.last_command);
    }

    if matches!(event.event_name, EventName::Stop) {
        session.last_assistant_message = assistant_message.or(session.last_assistant_message);
    }

    session.status = reduce_status(&session.status, event);
    session
}

fn reduce_status(previous: &SessionStatus, event: &CodexEvent) -> SessionStatus {
    match event.event_name {
        EventName::SessionStart => SessionStatus::Idle,
        EventName::UserPromptSubmit => SessionStatus::Working,
        EventName::PreToolUse => SessionStatus::Working,
        EventName::PermissionRequest => SessionStatus::WaitingApproval,
        EventName::PostToolUse => {
            if tool_response_failed(&event.payload) {
                SessionStatus::Errored
            } else {
                SessionStatus::Working
            }
        }
        EventName::Stop => {
            if extract_bool(&event.payload, "finished") == Some(true) {
                SessionStatus::Finished
            } else {
                SessionStatus::Idle
            }
        }
        EventName::Unknown => previous.clone(),
    }
}

pub fn reconcile_stale_status(session: &CodexSession, has_process: bool) -> SessionStatus {
    if has_process {
        return session.status.clone();
    }

    let Ok(updated_at) = DateTime::parse_from_rfc3339(&session.updated_at) else {
        return SessionStatus::Unknown;
    };
    let age = Utc::now().signed_duration_since(updated_at.with_timezone(&Utc));
    if age > Duration::minutes(30) {
        SessionStatus::Finished
    } else if age > Duration::minutes(10) {
        SessionStatus::Unknown
    } else {
        session.status.clone()
    }
}

fn tool_response_failed(payload: &Value) -> bool {
    let Some(tool_response) = payload.get("tool_response") else {
        return false;
    };

    if tool_response.get("success").and_then(Value::as_bool) == Some(false) {
        return true;
    }
    if tool_response
        .get("exit_code")
        .and_then(Value::as_i64)
        .unwrap_or(0)
        != 0
    {
        return true;
    }
    let serialized = tool_response.to_string().to_ascii_lowercase();
    serialized.contains("error") || serialized.contains("failed")
}

fn extract_command(payload: &Value) -> Option<String> {
    payload
        .get("tool_input")
        .and_then(|value| value.get("command"))
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn extract_string(payload: &Value, key: &str) -> Option<String> {
    payload.get(key).and_then(Value::as_str).map(str::to_string)
}

fn extract_bool(payload: &Value, key: &str) -> Option<bool> {
    payload.get(key).and_then(Value::as_bool)
}

fn status_to_string(status: &SessionStatus) -> &'static str {
    match status {
        SessionStatus::Working => "working",
        SessionStatus::Idle => "idle",
        SessionStatus::WaitingApproval => "waiting_approval",
        SessionStatus::Errored => "errored",
        SessionStatus::Finished => "finished",
        SessionStatus::Unknown => "unknown",
    }
}

fn status_from_string(status: &str) -> SessionStatus {
    match status {
        "working" => SessionStatus::Working,
        "idle" => SessionStatus::Idle,
        "waiting_approval" => SessionStatus::WaitingApproval,
        "errored" => SessionStatus::Errored,
        "finished" => SessionStatus::Finished,
        _ => SessionStatus::Unknown,
    }
}

fn event_name_to_string(event_name: &EventName) -> &'static str {
    match event_name {
        EventName::SessionStart => "SessionStart",
        EventName::UserPromptSubmit => "UserPromptSubmit",
        EventName::PreToolUse => "PreToolUse",
        EventName::PermissionRequest => "PermissionRequest",
        EventName::PostToolUse => "PostToolUse",
        EventName::Stop => "Stop",
        EventName::Unknown => "Unknown",
    }
}

fn event_name_from_string(event_name: &str) -> EventName {
    match event_name {
        "SessionStart" => EventName::SessionStart,
        "UserPromptSubmit" => EventName::UserPromptSubmit,
        "PreToolUse" => EventName::PreToolUse,
        "PermissionRequest" => EventName::PermissionRequest,
        "PostToolUse" => EventName::PostToolUse,
        "Stop" => EventName::Stop,
        _ => EventName::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use crate::{normalize_hook_payload, parse_hook_stdin};
    use chrono::{Duration, Utc};

    use super::{reconcile_stale_status, reduce_status};
    use crate::models::{EventName, SessionStatus};

    #[test]
    fn status_reducer_marks_post_tool_failure_as_errored() {
        let value = parse_hook_stdin(
      r#"{"session_id":"s-1","cwd":"/tmp","hook_event_name":"PostToolUse","tool_response":{"success":false}}"#,
    )
    .expect("parses json");
        let event = normalize_hook_payload(value).expect("normalizes event");
        let status = reduce_status(&SessionStatus::Working, &event);
        assert_eq!(status, SessionStatus::Errored);
    }

    #[test]
    fn stale_sessions_become_unknown_when_process_disappears() {
        let session = crate::models::CodexSession {
            id: "s-1".to_string(),
            cwd: "/tmp".to_string(),
            repo_root: None,
            repo_name: None,
            branch: None,
            model: None,
            transcript_path: None,
            status: SessionStatus::Idle,
            last_prompt: None,
            last_command: None,
            last_assistant_message: None,
            started_at: Utc::now().to_rfc3339(),
            updated_at: (Utc::now() - Duration::minutes(11)).to_rfc3339(),
        };
        let reconciled = reconcile_stale_status(&session, false);
        assert!(matches!(
            reconciled,
            SessionStatus::Unknown | SessionStatus::Finished
        ));
    }

    #[test]
    fn event_name_string_roundtrip_is_consistent() {
        assert_eq!(super::event_name_to_string(&EventName::Stop), "Stop");
    }
}
