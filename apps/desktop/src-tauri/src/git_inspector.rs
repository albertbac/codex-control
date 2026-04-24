use std::path::Path;
use std::process::Command;

use codex_core::GitSnapshot;

pub fn inspect_git(cwd: &str) -> GitSnapshot {
    if !Path::new(cwd).exists() {
        return GitSnapshot {
            changed_files_count: 0,
            staged_count: 0,
            unstaged_count: 0,
            diff_stat: None,
        };
    }

    let status_output = Command::new("git")
        .args(["-C", cwd, "status", "--porcelain", "--branch"])
        .output();

    let mut changed_files_count = 0usize;
    let mut staged_count = 0usize;
    let mut unstaged_count = 0usize;

    if let Ok(output) = status_output {
        let text = String::from_utf8_lossy(&output.stdout);
        for line in text.lines() {
            if line.starts_with("##") || line.trim().is_empty() {
                continue;
            }
            changed_files_count += 1;
            let bytes = line.as_bytes();
            if bytes.first().copied().unwrap_or(b' ') != b' '
                && bytes.first().copied().unwrap_or(b' ') != b'?'
            {
                staged_count += 1;
            }
            if bytes.get(1).copied().unwrap_or(b' ') != b' ' {
                unstaged_count += 1;
            }
        }
    }

    let diff_stat = Command::new("git")
        .args([
            "-C",
            cwd,
            "diff",
            "--stat",
            "--compact-summary",
            "--no-ext-diff",
        ])
        .output()
        .ok()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .filter(|value| !value.is_empty());

    GitSnapshot {
        changed_files_count,
        staged_count,
        unstaged_count,
        diff_stat,
    }
}

pub fn inspect_diff_preview(cwd: &str) -> String {
    if !Path::new(cwd).exists() {
        return "Workspace path is unavailable.".to_string();
    }

    let diff = Command::new("git")
        .args([
            "-C",
            cwd,
            "diff",
            "--stat",
            "--compact-summary",
            "--no-ext-diff",
        ])
        .output()
        .ok()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_default();

    if !diff.is_empty() {
        return diff;
    }

    Command::new("git")
        .args(["-C", cwd, "status", "--short", "--branch"])
        .output()
        .ok()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "No git diff available for this workspace.".to_string())
}
