use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use codex_core::{parse_transcript, TranscriptSummary};

use crate::redaction::sanitize_preview;

pub fn read_transcript_summary(path: Option<&str>) -> Option<TranscriptSummary> {
    path.map(Path::new).map(parse_transcript)
}

pub fn inspect_transcript(path: &str) -> Result<String> {
    let content = fs::read_to_string(path).context("unable to read transcript")?;
    let preview = content.lines().take(160).collect::<Vec<_>>().join("\n");
    Ok(sanitize_preview(&preview))
}
