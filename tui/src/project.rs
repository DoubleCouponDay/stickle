use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

const TEST_DIRS: [&str; 4] = ["tests", "test", "Tests", "Test"];

pub struct Project {
    pub root: PathBuf,
    pub test_project: Option<PathBuf>,
    pub framework: Option<u32>,
}

impl Project {
    pub fn discover() -> Self {
        let root = find_root();
        let test_project = find_test_project(&root);

        let framework = test_project
            .as_ref()
            .and_then(|relative| target_framework(&root.join(relative)));

        Project {
            root,
            test_project,
            framework,
        }
    }
}

fn find_root() -> PathBuf {
    let start = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut cursor = start.as_path();
    let mut fallback = None;

    loop {
        if cursor.join(".git").exists() {
            return cursor.to_path_buf();
        }

        if fallback.is_none() && csproj_in(cursor).is_some() {
            fallback = Some(cursor.to_path_buf());
        }

        match cursor.parent() {
            Some(parent) => cursor = parent,
            None => break,
        }
    }

    fallback.unwrap_or(start)
}

fn find_test_project(root: &Path) -> Option<PathBuf> {
    if let Some(name) = csproj_in(root) {
        return Some(PathBuf::from(name));
    }

    for candidate in TEST_DIRS {
        if let Some(name) = csproj_in(&root.join(candidate)) {
            return Some(Path::new(candidate).join(name));
        }
    }

    let mut found = Vec::new();

    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            if let Some(name) = csproj_in(&path) {
                found.push(Path::new(&entry.file_name()).join(name));
            }
        }
    }

    found.sort();
    found.into_iter().next()
}

fn csproj_in(dir: &Path) -> Option<OsString> {
    let mut names: Vec<OsString> = fs::read_dir(dir)
        .ok()?
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("csproj"))
        })
        .filter_map(|path| path.file_name().map(OsString::from))
        .collect();

    names.sort();
    names.into_iter().next()
}

fn target_framework(path: &Path) -> Option<u32> {
    let text = fs::read_to_string(path).ok()?;
    let mut newest = None;

    for tag in ["<TargetFramework>", "<TargetFrameworks>"] {
        let Some(start) = text.find(tag) else {
            continue;
        };

        let rest = &text[start + tag.len()..];

        let Some(end) = rest.find("</") else {
            continue;
        };

        for value in rest[..end].split(';') {
            if let Some(major) = major_version(value.trim()) {
                newest = Some(newest.map_or(major, |current: u32| current.max(major)));
            }
        }
    }

    newest
}

fn major_version(value: &str) -> Option<u32> {
    let digits: String = value
        .trim_start_matches("net")
        .chars()
        .take_while(char::is_ascii_digit)
        .collect();

    digits.parse().ok()
}
