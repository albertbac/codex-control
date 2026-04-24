use std::process::Command;

use codex_core::ProcessSnapshot;

pub fn collect_codex_processes() -> Vec<ProcessSnapshot> {
    let output = Command::new("ps")
        .args(["-axo", "pid=,ppid=,etimes=,command="])
        .output();

    let Ok(output) = output else {
        return Vec::new();
    };

    let text = String::from_utf8_lossy(&output.stdout);
    let mut processes = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let mut parts = trimmed.split_whitespace();
        let pid = parts.next().and_then(|value| value.parse::<i64>().ok());
        let parent_pid = parts.next().and_then(|value| value.parse::<i64>().ok());
        let uptime_seconds = parts.next().and_then(|value| value.parse::<u64>().ok());
        let command = parts.collect::<Vec<_>>().join(" ");
        let normalized = command.to_ascii_lowercase();
        if !normalized.contains("codex") || normalized.contains("codex-control") {
            continue;
        }
        let Some(pid) = pid else {
            continue;
        };
        processes.push(ProcessSnapshot {
            pid,
            parent_pid,
            cwd: resolve_cwd(pid).unwrap_or_default(),
            command,
            uptime_seconds: uptime_seconds.unwrap_or_default(),
        });
    }
    processes
}

fn resolve_cwd(pid: i64) -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        let path = std::fs::read_link(format!("/proc/{pid}/cwd")).ok()?;
        return Some(path.to_string_lossy().to_string());
    }

    #[cfg(target_os = "macos")]
    {
        let output = Command::new("lsof")
            .args(["-a", "-d", "cwd", "-p", &pid.to_string(), "-Fn"])
            .output()
            .ok()?;
        let text = String::from_utf8_lossy(&output.stdout);
        return text
            .lines()
            .find_map(|line| line.strip_prefix('n').map(str::to_string));
    }

    #[allow(unreachable_code)]
    None
}
