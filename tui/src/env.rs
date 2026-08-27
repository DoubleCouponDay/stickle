use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

pub struct EnvSnapshot {
    pub path: Vec<PathBuf>,
    #[cfg_attr(not(windows), allow(dead_code))]
    pub lib: Vec<PathBuf>,
    #[cfg_attr(not(windows), allow(dead_code))]
    pub lib_defined: bool,
    #[cfg_attr(windows, allow(dead_code))]
    pub library_path: Vec<PathBuf>,
    pub source: &'static str,
    #[cfg_attr(not(windows), allow(dead_code))]
    pub missing_here: Vec<String>,
    #[cfg_attr(not(windows), allow(dead_code))]
    pub session_only: Vec<String>,
}

impl EnvSnapshot {
    pub fn read() -> Self {
        platform::read()
    }

    #[cfg_attr(not(windows), allow(dead_code))]
    pub fn is_stale(&self) -> bool {
        !self.missing_here.is_empty()
    }
}

fn split(value: &OsString) -> Vec<PathBuf> {
    env::split_paths(value)
        .filter(|path| !path.as_os_str().is_empty())
        .collect()
}

fn process_var(name: &str) -> Option<Vec<PathBuf>> {
    env::var_os(name).map(|value| split(&value))
}

#[cfg(windows)]
fn normalise(path: &PathBuf) -> String {
    let text = path.display().to_string();
    let trimmed = text.trim_end_matches(['/', '\\']).to_string();

    if cfg!(windows) {
        trimmed.to_ascii_lowercase().replace('/', "\\")
    } else {
        trimmed
    }
}

#[cfg(windows)]
fn compare(
    label: &str,
    live: &[PathBuf],
    process: &[PathBuf],
    missing_here: &mut Vec<String>,
    session_only: &mut Vec<String>,
) {
    let live_keys: Vec<String> = live.iter().map(normalise).collect();
    let process_keys: Vec<String> = process.iter().map(normalise).collect();

    for (index, key) in live_keys.iter().enumerate() {
        if !process_keys.contains(key) {
            push_once(
                missing_here,
                format!("{label} {}", live[index].display()),
            );
        }
    }

    for (index, key) in process_keys.iter().enumerate() {
        if !live_keys.contains(key) {
            push_once(
                session_only,
                format!("{label} {}", process[index].display()),
            );
        }
    }
}

#[cfg(windows)]
fn push_once(into: &mut Vec<String>, entry: String) {
    if !into.contains(&entry) {
        into.push(entry);
    }
}

#[cfg(windows)]
mod platform {
    use std::collections::HashMap;
    use std::env;
    use std::ffi::OsString;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    use super::{EnvSnapshot, compare, process_var, split};

    const SYSTEM_KEY: &str =
        r"HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Environment";
    const USER_KEY: &str = r"HKCU\Environment";

    pub fn read() -> EnvSnapshot {
        let system = registry_values(SYSTEM_KEY);
        let user = registry_values(USER_KEY);

        let process_path = process_var("PATH").unwrap_or_default();
        let process_lib = process_var("LIB");

        let registry_path = joined(&system, &user, "PATH");
        let registry_lib = joined(&system, &user, "LIB");

        let mut missing_here = Vec::new();
        let mut session_only = Vec::new();

        let (path, lib, lib_defined, source) = match registry_path {
            Some(value) => {
                let path = split(&OsString::from(value));
                let lib_defined = registry_lib.is_some();
                let lib = registry_lib
                    .map(|value| split(&OsString::from(value)))
                    .unwrap_or_default();

                compare(
                    "PATH",
                    &path,
                    &process_path,
                    &mut missing_here,
                    &mut session_only,
                );
                compare(
                    "LIB",
                    &lib,
                    &process_lib.clone().unwrap_or_default(),
                    &mut missing_here,
                    &mut session_only,
                );

                (path, lib, lib_defined, "machine and user registry")
            }
            None => (
                process_path.clone(),
                process_lib.clone().unwrap_or_default(),
                process_lib.is_some(),
                "this process, the registry could not be read",
            ),
        };

        EnvSnapshot {
            path,
            lib,
            lib_defined,
            library_path: Vec::new(),
            source,
            missing_here,
            session_only,
        }
    }

    fn joined(
        system: &HashMap<String, String>,
        user: &HashMap<String, String>,
        name: &str,
    ) -> Option<String> {
        let system_value = system.get(name).map(|value| expand(value));
        let user_value = user.get(name).map(|value| expand(value));

        match (system_value, user_value) {
            (Some(first), Some(second)) => Some(format!("{};{}", first.trim_end_matches(';'), second)),
            (Some(first), None) => Some(first),
            (None, Some(second)) => Some(second),
            (None, None) => None,
        }
    }

    fn registry_values(key: &str) -> HashMap<String, String> {
        let mut values = HashMap::new();

        let Some(output) = query(key) else {
            return values;
        };

        for line in output.lines() {
            let trimmed = line.trim_end();

            let Some(marker) = trimmed.find("REG_") else {
                continue;
            };

            let name = trimmed[..marker].trim();

            if name.is_empty() {
                continue;
            }

            let mut columns = trimmed[marker..].splitn(2, char::is_whitespace);
            columns.next();

            let data = columns.next().unwrap_or("").trim();

            values.insert(name.to_ascii_uppercase(), data.to_string());
        }

        values
    }

    fn query(key: &str) -> Option<String> {
        for program in reg_programs() {
            let Ok(output) = Command::new(&program).args(["query", key]).output() else {
                continue;
            };

            if output.status.success() {
                return Some(String::from_utf8_lossy(&output.stdout).to_string());
            }
        }

        None
    }

    fn reg_programs() -> Vec<PathBuf> {
        let mut programs = Vec::new();

        if let Some(root) = env::var_os("SystemRoot") {
            programs.push(Path::new(&root).join("System32").join("reg.exe"));
        }

        programs.push(PathBuf::from("reg.exe"));
        programs
    }

    fn expand(value: &str) -> String {
        let mut expanded = String::new();
        let mut rest = value;

        while let Some(start) = rest.find('%') {
            expanded.push_str(&rest[..start]);

            let after = &rest[start + 1..];

            let Some(end) = after.find('%') else {
                expanded.push('%');
                rest = after;
                break;
            };

            let name = &after[..end];

            match env::var(name) {
                Ok(found) => expanded.push_str(&found),
                Err(_) => expanded.push_str(&format!("%{name}%")),
            }

            rest = &after[end + 1..];
        }

        expanded.push_str(rest);
        expanded
    }
}

#[cfg(not(windows))]
mod platform {
    use super::{EnvSnapshot, process_var};

    pub fn read() -> EnvSnapshot {
        let lib = process_var("LIB");

        EnvSnapshot {
            path: process_var("PATH").unwrap_or_default(),
            lib_defined: lib.is_some(),
            lib: lib.unwrap_or_default(),
            library_path: process_var("LD_LIBRARY_PATH").unwrap_or_default(),
            source: "this process, exported shell changes need a restart",
            missing_here: Vec::new(),
            session_only: Vec::new(),
        }
    }
}
