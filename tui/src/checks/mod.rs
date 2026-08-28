use std::fs;
use std::path::{Path, PathBuf};

use crate::builds::TEST_PROJECT_CHECK;
use crate::env::EnvSnapshot;
use crate::probe::Probes;
use crate::project::Project;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows as platform;

#[cfg(not(windows))]
mod unix;
#[cfg(not(windows))]
use unix as platform;

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
    pub test_project: Option<PathBuf>,
    pub platform: &'static str,
    pub env: EnvSnapshot,
    pub groups: Vec<Group>,
}

impl Report {
    pub fn pending() -> Self {
        let project = Project::discover();

        Report {
            root: project.root,
            test_project: project.test_project,
            platform: platform::NAME,
            env: EnvSnapshot::empty(),
            groups: Vec::new(),
        }
    }

    pub fn run(env: &EnvSnapshot, probes: &mut Probes) -> Self {
        let project = Project::discover();

        let mut groups = platform::groups(env, probes, &project.root);
        groups.push(dotnet_group(env, probes, &project));
        groups.push(sources_group(&project.root));

        Report {
            root: project.root,
            test_project: project.test_project,
            platform: platform::NAME,
            env: env.clone(),
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

fn dotnet_group(env: &EnvSnapshot, probes: &mut Probes, project: &Project) -> Group {
    let mut checks = Vec::new();
    let wanted = project.framework;

    let name = match wanted {
        Some(major) => format!(".NET SDK {major} or newer"),
        None => ".NET SDK".to_string(),
    };

    let expected = match wanted {
        Some(major) => format!(
            ".NET SDK {major}.x, the target framework declared by the test project is net{major}.0"
        ),
        None => ".NET SDK, needed to build and run the test project".to_string(),
    };

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

            checks.push(match (newest, wanted) {
                (Some(major), Some(required)) if major < required => Check {
                    name,
                    status: Status::Fail,
                    summary: format!("newest SDK is {major}.x"),
                    expected,
                    found: sdks.join("\n"),
                    remedy: vec![
                        format!(
                            "The test project targets net{required}.0, which SDK {major}.x cannot build."
                        ),
                        format!("Install the .NET {required} SDK:"),
                        platform::DOTNET_REMEDY.into(),
                    ],
                },
                (Some(major), _) => Check {
                    name,
                    status: Status::Pass,
                    summary: format!("SDK {major}.x installed"),
                    expected,
                    found: sdks.join("\n"),
                    remedy: Vec::new(),
                },
                (None, _) => Check {
                    name,
                    status: Status::Fail,
                    summary: "no SDK installed".into(),
                    expected,
                    found: "dotnet --list-sdks returned nothing".into(),
                    remedy: vec![
                        "Only a runtime is installed, dotnet test needs the SDK.".into(),
                        "Install the .NET SDK:".into(),
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
                remedy: vec!["Install the .NET SDK:".into(), platform::DOTNET_REMEDY.into()],
            });

            checks.push(Check {
                name,
                status: Status::Fail,
                summary: "cannot be queried".into(),
                expected,
                found: "dotnet is not installed".into(),
                remedy: vec!["Install the .NET SDK first.".into()],
            });
        }
    }

    checks.push(test_project_check(project));

    Group {
        title: ".NET test host".into(),
        checks,
    }
}

fn test_project_check(project: &Project) -> Check {
    let expected =
        "a .csproj in the project root or in a tests folder, the project dotnet test runs".into();

    match &project.test_project {
        Some(relative) => {
            let framework = match project.framework {
                Some(major) => format!("target framework: net{major}.0"),
                None => "target framework could not be read".into(),
            };

            Check {
                name: TEST_PROJECT_CHECK.into(),
                status: Status::Pass,
                summary: relative.display().to_string(),
                expected,
                found: format!("{}\n{framework}", project.root.join(relative).display()),
                remedy: Vec::new(),
            }
        }
        None => Check {
            name: TEST_PROJECT_CHECK.into(),
            status: Status::Fail,
            summary: "no .csproj found".into(),
            expected,
            found: format!(
                "searched {} and its immediate subfolders",
                project.root.display()
            ),
            remedy: vec![
                "dotnet test needs a project file to run.".into(),
                "Put the test .csproj in the project root or in a tests folder.".into(),
                "Start this app from inside the checkout that holds it.".into(),
            ],
        },
    }
}

fn sources_group(root: &Path) -> Group {
    let mut checks = vec![
        st_dir_check(
            root,
            "source",
            "source .st files",
            "the Structured Text sources plc compiles into the shared library",
        ),
        st_dir_check(
            root,
            "libNX1P2",
            "libNX1P2 .st files",
            "the library sources plc compiles into libNX1P2, which the shared library links against",
        ),
        st_dir_check(
            root,
            "externals",
            "externals .st files",
            "the declaration files passed to plc with -i, without which the sources will not compile",
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
