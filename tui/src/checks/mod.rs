use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::env::EnvSnapshot;
use crate::probe::Probes;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows as platform;

#[cfg(not(windows))]
mod unix;
#[cfg(not(windows))]
use unix as platform;

pub const REQUIRED_DOTNET_MAJOR: u32 = 10;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Pass,
    #[cfg_attr(not(windows), allow(dead_code))]
    Warn,
    Fail,
}

impl Status {
    pub fn badge(self) -> &'static str {
        match self {
            Status::Pass => " MET  ",
            Status::Warn => " WARN ",
            Status::Fail => " UNMET",
        }
    }

    pub fn word(self) -> &'static str {
        match self {
            Status::Pass => "Requirement met",
            Status::Warn => "Requirement met with a mismatch",
            Status::Fail => "Requirement not met",
        }
    }
}

pub struct Check {
    pub name: String,
    pub status: Status,
    pub summary: String,
    pub expected: String,
    pub found: String,
    pub remedy: Vec<String>,
}

pub struct Group {
    pub title: String,
    pub checks: Vec<Check>,
}

pub struct Report {
    pub root: PathBuf,
    pub platform: &'static str,
    pub env_source: String,
    pub groups: Vec<Group>,
}

impl Report {
    pub fn pending() -> Self {
        Report {
            root: find_root(),
            platform: platform::NAME,
            env_source: "reading".into(),
            groups: Vec::new(),
        }
    }

    pub fn run(env: &EnvSnapshot, probes: &mut Probes) -> Self {
        let root = find_root();

        let mut groups = platform::groups(env, probes, &root);
        groups.push(dotnet_group(env, probes, &root));
        groups.push(sources_group(&root));

        Report {
            root,
            platform: platform::NAME,
            env_source: env.source.into(),
            groups,
        }
    }

    pub fn totals(&self) -> (usize, usize, usize, usize) {
        let mut pass = 0;
        let mut warn = 0;
        let mut fail = 0;

        for group in &self.groups {
            for check in &group.checks {
                match check.status {
                    Status::Pass => pass += 1,
                    Status::Warn => warn += 1,
                    Status::Fail => fail += 1,
                }
            }
        }

        (pass + warn + fail, pass, warn, fail)
    }
}

fn dotnet_group(env: &EnvSnapshot, probes: &mut Probes, root: &Path) -> Group {
    let mut checks = Vec::new();

    match which(env, "dotnet") {
        Some(path) => {
            checks.push(Check {
                name: "dotnet on PATH".into(),
                status: Status::Pass,
                summary: "available".into(),
                expected: "the dotnet driver resolvable through PATH, used by dotnet test".into(),
                found: path.display().to_string(),
                remedy: Vec::new(),
            });

            let sdks = probes.lines(&path, &["--list-sdks"]);
            let newest = sdks.iter().filter_map(|sdk| sdk_major(sdk)).max();

            checks.push(match newest {
                Some(major) if major >= REQUIRED_DOTNET_MAJOR => Check {
                    name: format!(".NET SDK {REQUIRED_DOTNET_MAJOR} or newer"),
                    status: Status::Pass,
                    summary: format!("SDK {major}.x installed"),
                    expected: format!(
                        ".NET SDK {REQUIRED_DOTNET_MAJOR}.x, stickle.csproj targets net{REQUIRED_DOTNET_MAJOR}.0"
                    ),
                    found: sdks.join("\n"),
                    remedy: Vec::new(),
                },
                Some(major) => Check {
                    name: format!(".NET SDK {REQUIRED_DOTNET_MAJOR} or newer"),
                    status: Status::Fail,
                    summary: format!("newest SDK is {major}.x"),
                    expected: format!(
                        ".NET SDK {REQUIRED_DOTNET_MAJOR}.x, stickle.csproj targets net{REQUIRED_DOTNET_MAJOR}.0"
                    ),
                    found: sdks.join("\n"),
                    remedy: vec![
                        format!(
                            "stickle.csproj targets net{REQUIRED_DOTNET_MAJOR}.0, which SDK {major}.x cannot build."
                        ),
                        format!("Install the .NET {REQUIRED_DOTNET_MAJOR} SDK:"),
                        platform::DOTNET_REMEDY.into(),
                    ],
                },
                None => Check {
                    name: format!(".NET SDK {REQUIRED_DOTNET_MAJOR} or newer"),
                    status: Status::Fail,
                    summary: "no SDK installed".into(),
                    expected: format!(".NET SDK {REQUIRED_DOTNET_MAJOR}.x"),
                    found: "dotnet --list-sdks returned nothing".into(),
                    remedy: vec![
                        format!("Only a runtime is installed, dotnet test needs the SDK."),
                        format!("Install the .NET {REQUIRED_DOTNET_MAJOR} SDK:"),
                        platform::DOTNET_REMEDY.into(),
                    ],
                },
            });
        }
        None => {
            checks.push(Check {
                name: "dotnet on PATH".into(),
                status: Status::Fail,
                summary: "not found on PATH".into(),
                expected: "the dotnet driver resolvable through PATH, used by dotnet test".into(),
                found: format!("no dotnet in any of the {} PATH folder(s)", env.path.len()),
                remedy: vec![
                    format!("Install the .NET {REQUIRED_DOTNET_MAJOR} SDK:"),
                    platform::DOTNET_REMEDY.into(),
                ],
            });

            checks.push(Check {
                name: format!(".NET SDK {REQUIRED_DOTNET_MAJOR} or newer"),
                status: Status::Fail,
                summary: "cannot be queried".into(),
                expected: format!(".NET SDK {REQUIRED_DOTNET_MAJOR}.x"),
                found: "dotnet is not installed".into(),
                remedy: vec!["Install the .NET SDK first.".into()],
            });
        }
    }

    checks.push(file_check(
        root,
        "stickle.csproj",
        "stickle.csproj",
        "the test project consumed by dotnet test",
        vec!["Run this app from inside a stickle checkout.".into()],
    ));

    Group {
        title: ".NET test host".into(),
        checks,
    }
}

fn sources_group(root: &Path) -> Group {
    let mut checks = vec![
        st_dir_check(
            root,
            "source",
            "source .st files",
            "the Structured Text sources compiled into the lib_structured_text library",
        ),
        st_dir_check(
            root,
            "libomron",
            "libomron .st files",
            "the Omron library sources compiled into the NX1P2 library",
        ),
        file_check(
            root,
            "externals/stdlib_externals.st",
            "externals/stdlib_externals.st",
            "the stdlib function block declarations passed with -i, without which source will not compile",
            vec!["Restore the file from source control.".into()],
        ),
        file_check(
            root,
            "externals/omron_externals.st",
            "externals/omron_externals.st",
            "the Omron system variable declarations passed with -i, without which source will not compile",
            vec!["Restore the file from source control.".into()],
        ),
    ];

    let compiled = root.join("compiled");

    checks.push(if compiled.is_dir() {
        Check {
            name: "compiled output folder".into(),
            status: Status::Pass,
            summary: "present".into(),
            expected: "the compiled folder, which plc and the linker write their output into".into(),
            found: compiled.display().to_string(),
            remedy: Vec::new(),
        }
    } else {
        Check {
            name: "compiled output folder".into(),
            status: Status::Fail,
            summary: "missing".into(),
            expected: "the compiled folder, which plc and the linker write their output into".into(),
            found: format!("{} does not exist", compiled.display()),
            remedy: vec![
                "The folder is ignored by git, so a fresh clone does not have it.".into(),
                "Neither plc nor the linker creates it, so create it yourself:".into(),
                "mkdir compiled".into(),
            ],
        }
    });

    Group {
        title: "Project sources".into(),
        checks,
    }
}

pub fn file_check(
    root: &Path,
    relative: &str,
    name: &str,
    purpose: &str,
    remedy: Vec<String>,
) -> Check {
    let path = root.join(relative);

    if path.is_file() {
        Check {
            name: name.into(),
            status: Status::Pass,
            summary: "present".into(),
            expected: format!("{name}, {purpose}"),
            found: path.display().to_string(),
            remedy: Vec::new(),
        }
    } else {
        Check {
            name: name.into(),
            status: Status::Fail,
            summary: "missing".into(),
            expected: format!("{name}, {purpose}"),
            found: format!("{} does not exist", path.display()),
            remedy,
        }
    }
}

fn st_dir_check(root: &Path, relative: &str, name: &str, purpose: &str) -> Check {
    let dir = root.join(relative);
    let files = st_files(&dir);

    if files.is_empty() {
        Check {
            name: name.into(),
            status: Status::Fail,
            summary: "no .st files".into(),
            expected: format!("{name}, {purpose}"),
            found: format!("{} holds no .st files", dir.display()),
            remedy: vec!["Restore the sources from source control.".into()],
        }
    } else {
        Check {
            name: name.into(),
            status: Status::Pass,
            summary: format!("{} file(s)", files.len()),
            expected: format!("{name}, {purpose}"),
            found: files.join("\n"),
            remedy: Vec::new(),
        }
    }
}

fn st_files(dir: &Path) -> Vec<String> {
    let mut found = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("st"))
            {
                found.push(path.display().to_string());
            }
        }
    }

    found.sort();
    found
}

fn find_root() -> PathBuf {
    let start = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut cursor = start.as_path();

    loop {
        if cursor.join("stickle.csproj").is_file() {
            return cursor.to_path_buf();
        }

        match cursor.parent() {
            Some(parent) => cursor = parent,
            None => return start,
        }
    }
}

pub fn which(env: &EnvSnapshot, name: &str) -> Option<PathBuf> {
    for dir in &env.path {
        for candidate in platform::candidates(dir, name) {
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }

    None
}

pub fn find_in(dirs: &[PathBuf], file: &str) -> Option<PathBuf> {
    dirs.iter()
        .map(|dir| dir.join(file))
        .find(|candidate| candidate.is_file())
}

fn sdk_major(line: &str) -> Option<u32> {
    line.split(['.', ' ']).next()?.parse().ok()
}
