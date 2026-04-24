use std::path::PathBuf;
use std::{env, fs};

use anyhow::{Context, Result};
use directories::ProjectDirs;

use crate::models::DataPaths;

pub fn application_paths() -> Result<DataPaths> {
    if let Ok(value) = env::var("CODEX_CONTROL_DATA_DIR") {
        if !value.trim().is_empty() {
            let data_dir = PathBuf::from(value);
            let database_path = data_dir.join("codex-control.db");
            let spool_path = data_dir.join("spool").join("events.jsonl");
            return Ok(DataPaths {
                data_dir: data_dir.to_string_lossy().to_string(),
                database_path: database_path.to_string_lossy().to_string(),
                spool_path: spool_path.to_string_lossy().to_string(),
            });
        }
    }

    let project_dirs = ProjectDirs::from("com", "CodexControl", "CodexControl")
        .context("unable to resolve local application directory")?;
    let data_dir = project_dirs.data_local_dir().to_path_buf();
    let database_path = data_dir.join("codex-control.db");
    let spool_path = data_dir.join("spool").join("events.jsonl");

    Ok(DataPaths {
        data_dir: data_dir.to_string_lossy().to_string(),
        database_path: database_path.to_string_lossy().to_string(),
        spool_path: spool_path.to_string_lossy().to_string(),
    })
}

pub fn ensure_data_dirs() -> Result<DataPaths> {
    let paths = application_paths()?;
    fs::create_dir_all(&paths.data_dir).context("unable to create local data directory")?;
    let spool_parent = PathBuf::from(&paths.spool_path)
        .parent()
        .map(PathBuf::from)
        .context("unable to resolve spool parent directory")?;
    fs::create_dir_all(spool_parent).context("unable to create spool directory")?;
    Ok(paths)
}
