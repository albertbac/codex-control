use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoContext {
    pub repo_root: Option<String>,
    pub repo_name: Option<String>,
    pub branch: Option<String>,
}

pub fn discover_repo_context(cwd: &str) -> RepoContext {
    let start = Path::new(cwd);
    let repo_root = find_repo_root(start);
    let branch = repo_root
        .as_ref()
        .and_then(|root| read_branch_name(root).ok())
        .flatten();

    RepoContext {
        repo_root: repo_root
            .as_ref()
            .map(|path| path.to_string_lossy().to_string()),
        repo_name: repo_root
            .as_ref()
            .and_then(|path| path.file_name())
            .map(|name| name.to_string_lossy().to_string()),
        branch,
    }
}

fn find_repo_root(start: &Path) -> Option<PathBuf> {
    let mut current = if start.is_dir() {
        Some(start.to_path_buf())
    } else {
        start.parent().map(Path::to_path_buf)
    };

    while let Some(path) = current {
        let dot_git = path.join(".git");
        if dot_git.exists() {
            return Some(path);
        }
        current = path.parent().map(Path::to_path_buf);
    }

    None
}

fn read_branch_name(repo_root: &Path) -> std::io::Result<Option<String>> {
    let git_dir = resolve_git_dir(repo_root)?;
    let head = fs::read_to_string(git_dir.join("HEAD"))?;
    if let Some(reference) = head.trim().strip_prefix("ref: ") {
        return Ok(reference.rsplit('/').next().map(str::to_string));
    }
    Ok(None)
}

fn resolve_git_dir(repo_root: &Path) -> std::io::Result<PathBuf> {
    let dot_git = repo_root.join(".git");
    if dot_git.is_dir() {
        return Ok(dot_git);
    }

    let pointer = fs::read_to_string(&dot_git)?;
    if let Some(path) = pointer.trim().strip_prefix("gitdir: ") {
        let candidate = Path::new(path);
        if candidate.is_absolute() {
            Ok(candidate.to_path_buf())
        } else {
            Ok(repo_root.join(candidate))
        }
    } else {
        Ok(dot_git)
    }
}
