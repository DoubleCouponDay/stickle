use std::env;
use std::ffi::OsString;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::time::Instant;

use crate::checks::which;
use crate::env::EnvSnapshot;

#[derive(Clone)]
pub struct Step {
    pub program: &'static str,
    pub args: Vec<String>,
}

#[derive(Clone)]
pub struct Target {
    pub label: &'static str,
    pub ignores: &'static [&'static str],
    pub requires: &'static [&'static str],
    pub steps: Vec<Step>,
}

pub const SOURCE_CHECK: &str = "source .st files";
pub const BUILTINS_SOURCE_CHECK: &str = "libslinked .st files";
pub const TEST_PROJECT_CHECK: &str = "tests folder with .csproj";
pub const DOTNET_GROUP: &str = ".NET test host";
pub const PROJECT_TOKEN: &str = "{project}";
pub const LIBRARIES_TOKEN: &str = "{libraries}";
pub const LIBRARIES_DIR: &str = "libslinked";

impl Target {
    pub fn resolved(&self, project: Option<&Path>, root: &Path) -> Target {
        let mut copy = self.clone();
        let libraries = linked_libraries(root);

        for step in &mut copy.steps {
            step.args = step
                .args
                .iter()
                .flat_map(|arg| {
                    if arg == PROJECT_TOKEN {
                        project.map(|path| path.display().to_string()).into_iter().collect()
                    } else if arg == LIBRARIES_TOKEN {
                        libraries
                            .iter()
                            .flat_map(|name| ["-l".to_string(), name.clone()])
                            .collect()
                    } else {
                        vec![arg.clone()]
                    }
                })
                .collect();
        }

        copy
    }
}

pub fn linked_libraries(root: &Path) -> Vec<String> {
    let mut names = vec![BUILTINS_LINK_NAME.to_string()];

    let Ok(entries) = std::fs::read_dir(root.join(LIBRARIES_DIR)) else {
        return names;
    };

    let mut extra: Vec<String> = entries
        .filter_map(|entry| entry.ok().map(|it| it.path()))
        .filter(|path| {
            path.is_file()
                && path
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case(SHARED_EXTENSION))
        })
        .filter_map(|path| path.file_stem().map(|stem| stem.to_string_lossy().into_owned()))
        .map(|stem| link_name(&stem))
        .filter(|name| !name.eq_ignore_ascii_case(BUILTINS_LINK_NAME))
        .collect();

    extra.sort_by_key(|name| name.to_lowercase());
    extra.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    names.extend(extra);
    names
}

#[cfg(windows)]
fn link_name(stem: &str) -> String {
    stem.to_string()
}

#[cfg(not(windows))]
fn link_name(stem: &str) -> String {
    stem.strip_prefix("lib").unwrap_or(stem).to_string()
}

#[cfg(windows)]
pub const BUILTINS_CHECK: &str = "libbuiltins.dll";
#[cfg(windows)]
pub const BUILTINS_ARTIFACTS: &[&str] = &[
    "compiled/libbuiltins.dll",
    "libslinked/libbuiltins.dll",
];
#[cfg(windows)]
pub const LIBRARY_ARTIFACTS: &[&str] = &["compiled/lib_structured_text.dll"];
#[cfg(windows)]
pub const BUILTINS_LINK_NAME: &str = "libbuiltins";
#[cfg(windows)]
pub const SHARED_EXTENSION: &str = "dll";

#[cfg(not(windows))]
pub const BUILTINS_CHECK: &str = "libbuiltins.so";
#[cfg(not(windows))]
pub const BUILTINS_ARTIFACTS: &[&str] = &["compiled/libbuiltins.so", "libslinked/libbuiltins.so"];
#[cfg(not(windows))]
pub const LIBRARY_ARTIFACTS: &[&str] = &["compiled/lib_structured_text.so"];
#[cfg(not(windows))]
pub const BUILTINS_LINK_NAME: &str = "builtins";
#[cfg(not(windows))]
pub const SHARED_EXTENSION: &str = "so";

#[derive(Clone, Copy)]
pub enum State {
    Idle,
    Running,
    Done {
        ok: bool,
        seconds: f64,
        code: Option<i32>,
    },
}

pub struct Outcome {
    pub index: usize,
    pub ok: bool,
    pub code: Option<i32>,
    pub message: String,
    pub seconds: f64,
}

pub struct Runner {
    outcomes: Receiver<Outcome>,
    finished: Sender<Outcome>,
    printed: Receiver<String>,
    prints: Sender<String>,
    pub running: Option<usize>,
}

impl Runner {
    pub fn new() -> Self {
        let (finished, outcomes) = mpsc::channel();
        let (prints, printed) = mpsc::channel();

        Runner {
            outcomes,
            finished,
            printed,
            prints,
            running: None,
        }
    }

    pub fn start(&mut self, index: usize, target: &Target, root: PathBuf, snapshot: EnvSnapshot) {
        let label = target.label;
        let steps = target.steps.clone();
        let finished = self.finished.clone();
        let prints = self.prints.clone();

        self.running = Some(index);

        thread::spawn(move || {
            let started = Instant::now();
            let mut message = String::new();
            let mut ok = true;
            let mut code = None;

            for step in steps {
                let Some(program) = which(&snapshot, step.program) else {
                    ok = false;
                    message = format!("{} was not found on PATH", step.program);
                    let _ = prints.send(message.clone());
                    break;
                };

                let mut command = Command::new(&program);
                command
                    .args(&step.args)
                    .current_dir(&root)
                    .stdin(Stdio::null())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped());

                if let Some(value) = join(&snapshot.path) {
                    command.env("PATH", value);
                }

                if let Some(value) = join(&snapshot.lib) {
                    command.env("LIB", value);
                }

                let _ = prints.send(format!("$ {} {}", step.program, step.args.join(" ")));

                let mut child = match command.spawn() {
                    Ok(child) => child,
                    Err(error) => {
                        ok = false;
                        message = format!("{}: {error}", step.program);
                        let _ = prints.send(message.clone());
                        break;
                    }
                };

                let out = child.stdout.take().map(|pipe| pump(pipe, prints.clone()));
                let err = child.stderr.take().map(|pipe| pump(pipe, prints.clone()));

                let status = child.wait();

                let stdout_tail = out.and_then(|handle| handle.join().ok()).flatten();
                let stderr_tail = err.and_then(|handle| handle.join().ok()).flatten();

                if let Some(line) = stderr_tail.or(stdout_tail) {
                    message = line;
                }

                match status {
                    Ok(status) if status.success() => {}
                    Ok(status) => {
                        ok = false;
                        code = status.code();

                        if message.is_empty() {
                            message = format!("{} failed", step.program);
                        }

                        break;
                    }
                    Err(error) => {
                        ok = false;
                        message = format!("{}: {error}", step.program);
                        let _ = prints.send(message.clone());
                        break;
                    }
                }
            }

            if ok && message.is_empty() {
                message = format!("{label} finished");
            }

            let _ = finished.send(Outcome {
                index,
                ok,
                code,
                message,
                seconds: started.elapsed().as_secs_f64(),
            });
        });
    }

    pub fn poll(&mut self) -> Option<Outcome> {
        let outcome = self.outcomes.try_recv().ok()?;

        if self.running == Some(outcome.index) {
            self.running = None;
        }

        Some(outcome)
    }

    pub fn drain(&mut self, limit: usize) -> Vec<String> {
        let mut batch = Vec::new();

        while batch.len() < limit {
            match self.printed.try_recv() {
                Ok(line) => batch.push(line),
                Err(_) => break,
            }
        }

        batch
    }
}

fn pump<R>(reader: R, prints: Sender<String>) -> JoinHandle<Option<String>>
where
    R: Read + Send + 'static,
{
    thread::spawn(move || {
        let mut buffered = BufReader::new(reader);
        let mut raw = Vec::new();
        let mut last = None;

        loop {
            raw.clear();

            match buffered.read_until(b'\n', &mut raw) {
                Ok(0) | Err(_) => return last,
                Ok(_) => {}
            }

            let text = plain(String::from_utf8_lossy(&raw).trim_end_matches(['\r', '\n']));

            if !text.trim().is_empty() {
                last = Some(text.clone());
            }

            if prints.send(text).is_err() {
                return last;
            }
        }
    })
}

fn plain(text: &str) -> String {
    let mut stripped = String::with_capacity(text.len());
    let mut characters = text.chars();

    while let Some(character) = characters.next() {
        if character != '\u{1b}' {
            if !character.is_control() || character == '\t' {
                stripped.push(character);
            }

            continue;
        }

        match characters.next() {
            Some('[') => {
                for next in characters.by_ref() {
                    if ('\u{40}'..='\u{7e}').contains(&next) {
                        break;
                    }
                }
            }
            Some(']') => {
                while let Some(next) = characters.next() {
                    if next == '\u{7}' {
                        break;
                    }

                    if next == '\u{1b}' {
                        characters.next();
                        break;
                    }
                }
            }
            _ => {}
        }
    }

    stripped
}

fn join(dirs: &[PathBuf]) -> Option<OsString> {
    if dirs.is_empty() {
        return None;
    }

    env::join_paths(dirs).ok()
}

#[cfg(windows)]
pub fn targets() -> Vec<Target> {
    vec![
        Target {
            label: "libbuiltins.dll",
            ignores: &[SOURCE_CHECK, BUILTINS_CHECK, DOTNET_GROUP],
            requires: &[],
            steps: vec![
                Step {
                    program: "plc",
                    args: args(&[
                        "./libslinked/**/*.st",
                        "-c",
                        "-l",
                        "iec61131std",
                        "-l",
                        "ws2_32",
                        "-l",
                        "ntdll",
                        "-l",
                        "userenv",
                        "-o",
                        "./compiled/libbuiltins.o",
                    ]),
                },
                Step {
                    program: "clang",
                    args: args(&[
                        "./compiled/libbuiltins.o",
                        "--shared",
                        "-l",
                        "iec61131std",
                        "-l",
                        "ws2_32",
                        "-l",
                        "ntdll",
                        "-l",
                        "userenv",
                        "-fuse-ld=lld-link",
                        "-Wl,/DEF:libslinked/exports.def",
                        "-o",
                        "./compiled/libbuiltins.dll",
                    ]),
                },
            ],
        },
        Target {
            label: "lib_structured_text.dll",
            ignores: &[BUILTINS_SOURCE_CHECK, DOTNET_GROUP],
            requires: BUILTINS_ARTIFACTS,
            steps: vec![
                Step {
                    program: "plc",
                    args: args(&[
                        "./source/**/*.st",
                        "-c",
                        "-i",
                        "./**/externals/*.st",
                        "-L",
                        "./compiled",
                        "-L",
                        "./libslinked",
                        "-l",
                        "iec61131std",
                        LIBRARIES_TOKEN,
                        "-l",
                        "ws2_32",
                        "-l",
                        "ntdll",
                        "-l",
                        "userenv",
                        "-o",
                        "./compiled/lib_structured_text.o",
                    ]),
                },
                Step {
                    program: "clang",
                    args: args(&[
                        "./compiled/lib_structured_text.o",
                        "--shared",
                        "-l",
                        "iec61131std",
                        "-L",
                        "./compiled",
                        "-L",
                        "./libslinked",
                        LIBRARIES_TOKEN,
                        "-l",
                        "ws2_32",
                        "-l",
                        "ntdll",
                        "-l",
                        "userenv",
                        "-fuse-ld=lld-link",
                        "-Wl,/DEF:exports.def",
                        "-o",
                        "./compiled/lib_structured_text.dll",
                    ]),
                },
            ],
        },
        Target {
            label: "lib_structured_text.xml",
            ignores: &[BUILTINS_SOURCE_CHECK, DOTNET_GROUP],
            requires: BUILTINS_ARTIFACTS,
            steps: vec![Step {
                program: "plc",
                args: args(&[
                    "./source/**/*.st",
                    "--xml-omron",
                    "-i",
                    "./**/externals/*.st",
                    "-L",
                    "./compiled",
                    "-L",
                    "./libslinked",
                    "-l",
                    "iec61131std",
                    LIBRARIES_TOKEN,
                    "-l",
                    "ws2_32",
                    "-l",
                    "ntdll",
                    "-l",
                    "userenv",
                    "-o",
                    "./compiled/lib_structured_text.xml",
                ]),
            }],
        },
        test_target(),
    ]
}

fn test_target() -> Target {
    Target {
        label: "dotnet test",
        ignores: &[],
        requires: LIBRARY_ARTIFACTS,
        steps: vec![Step {
            program: "dotnet",
            args: args(&[
                "test",
                PROJECT_TOKEN,
                "--logger",
                "console;verbosity=detailed",
            ]),
        }],
    }
}

#[cfg(not(windows))]
pub fn targets() -> Vec<Target> {
    vec![
        Target {
            label: "libbuiltins.so",
            ignores: &[SOURCE_CHECK, BUILTINS_CHECK, DOTNET_GROUP],
            requires: &[],
            steps: vec![Step {
                program: "plc",
                args: args(&[
                    "./libslinked/**/*.st",
                    "--shared",
                    "--linker=cc",
                    "-L",
                    "./compiled",
                    "-l",
                    "iec61131std",
                    "-o",
                    "./compiled/libbuiltins.so",
                ]),
            }],
        },
        Target {
            label: "lib_structured_text.so",
            ignores: &[BUILTINS_SOURCE_CHECK, DOTNET_GROUP],
            requires: BUILTINS_ARTIFACTS,
            steps: vec![Step {
                program: "plc",
                args: args(&[
                    "./source/**/*.st",
                    "--shared",
                    "--linker=cc",
                    "--generate-external-constructors",
                    "-i",
                    "./**/externals/*.st",
                    "-L",
                    "./compiled",
                    "-L",
                    "./libslinked",
                    "-l",
                    "iec61131std",
                    LIBRARIES_TOKEN,
                    "--linker-arg=--rpath=$ORIGIN",
                    "-o",
                    "./compiled/lib_structured_text.so",
                ]),
            }],
        },
        Target {
            label: "lib_structured_text.xml",
            ignores: &[BUILTINS_SOURCE_CHECK, DOTNET_GROUP],
            requires: BUILTINS_ARTIFACTS,
            steps: vec![Step {
                program: "plc",
                args: args(&[
                    "./source/**/*.st",
                    "--xml-omron",
                    "-i",
                    "./**/externals/*.st",
                    "-L",
                    "./compiled",
                    "-L",
                    "./libslinked",
                    "-l",
                    "iec61131std",
                    LIBRARIES_TOKEN,
                    "-o",
                    "./compiled/lib_structured_text.xml",
                ]),
            }],
        },
        test_target(),
    ]
}

fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}
