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
    pub requires: Option<&'static str>,
    pub steps: Vec<Step>,
}

pub const SOURCE_CHECK: &str = "source .st files";
pub const TEST_PROJECT_CHECK: &str = "tests folder with .csproj";
pub const PROJECT_TOKEN: &str = "{project}";

impl Target {
    pub fn resolved(&self, project: Option<&Path>) -> Target {
        let mut copy = self.clone();

        for step in &mut copy.steps {
            step.args = step
                .args
                .iter()
                .filter_map(|arg| {
                    if arg == PROJECT_TOKEN {
                        project.map(|path| path.display().to_string())
                    } else {
                        Some(arg.clone())
                    }
                })
                .collect();
        }

        copy
    }
}

#[cfg(windows)]
pub const OMRON_CHECK: &str = "libNX1P2.dll";
#[cfg(windows)]
pub const OMRON_ARTIFACT: &str = "compiled/libNX1P2.dll";
#[cfg(windows)]
pub const LIBRARY_ARTIFACT: &str = "compiled/lib_structured_text.dll";

#[cfg(not(windows))]
pub const OMRON_CHECK: &str = "libNX1P2.so";
#[cfg(not(windows))]
pub const OMRON_ARTIFACT: &str = "compiled/libNX1P2.so";
#[cfg(not(windows))]
pub const LIBRARY_ARTIFACT: &str = "compiled/lib_structured_text.so";

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
            label: "libNX1P2.dll",
            ignores: &[SOURCE_CHECK, OMRON_CHECK, TEST_PROJECT_CHECK],
            requires: None,
            steps: vec![
                Step {
                    program: "plc",
                    args: args(&[
                        "./libNX1P2/*.st",
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
                        "./compiled/libNX1P2.o",
                    ]),
                },
                Step {
                    program: "clang",
                    args: args(&[
                        "./compiled/libNX1P2.o",
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
                        "-Wl,/DEF:libNX1P2/exports.def",
                        "-o",
                        "./compiled/libNX1P2.dll",
                    ]),
                },
            ],
        },
        Target {
            label: "lib_structured_text.dll",
            ignores: &[TEST_PROJECT_CHECK],
            requires: Some(OMRON_ARTIFACT),
            steps: vec![
                Step {
                    program: "plc",
                    args: args(&[
                        "./source/*.st",
                        "-c",
                        "-i",
                        "./externals/stdlib_externals.st",
                        "-i",
                        "./externals/omron_externals.st",
                        "-L",
                        "./compiled",
                        "-l",
                        "iec61131std",
                        "-l",
                        "libNX1P2",
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
                        "-l",
                        "libNX1P2",
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
            ignores: &[TEST_PROJECT_CHECK],
            requires: Some(OMRON_ARTIFACT),
            steps: vec![Step {
                program: "plc",
                args: args(&[
                    "./source/clampandsaw.st",
                    "./source/testallbuiltins.st",
                    "--xml-omron",
                    "-i",
                    "./externals/stdlib_externals.st",
                    "-i",
                    "./externals/omron_externals.st",
                    "-L",
                    "./compiled",
                    "-l",
                    "iec61131std",
                    "-l",
                    "libNX1P2",
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
        requires: Some(LIBRARY_ARTIFACT),
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
            label: "libNX1P2.so",
            ignores: &[SOURCE_CHECK, OMRON_CHECK, TEST_PROJECT_CHECK],
            requires: None,
            steps: vec![Step {
                program: "plc",
                args: args(&[
                    "./libNX1P2/*.st",
                    "--shared",
                    "--linker=cc",
                    "-l",
                    "iec61131std",
                    "-o",
                    "./compiled/libNX1P2.so",
                ]),
            }],
        },
        Target {
            label: "lib_structured_text.so",
            ignores: &[TEST_PROJECT_CHECK],
            requires: Some(OMRON_ARTIFACT),
            steps: vec![Step {
                program: "plc",
                args: args(&[
                    "./source/*.st",
                    "--shared",
                    "--linker=cc",
                    "-i",
                    "./externals/stdlib_externals.st",
                    "-i",
                    "./externals/omron_externals.st",
                    "-L",
                    "./compiled",
                    "-l",
                    "iec61131std",
                    "-l",
                    "NX1P2",
                    "--linker-arg=--rpath=$ORIGIN",
                    "-o",
                    "./compiled/lib_structured_text.so",
                ]),
            }],
        },
        Target {
            label: "lib_structured_text.xml",
            ignores: &[TEST_PROJECT_CHECK],
            requires: Some(OMRON_ARTIFACT),
            steps: vec![Step {
                program: "plc",
                args: args(&[
                    "./source/clampandsaw.st",
                    "./source/testallbuiltins.st",
                    "--xml-omron",
                    "-i",
                    "./externals/stdlib_externals.st",
                    "-i",
                    "./externals/omron_externals.st",
                    "-L",
                    "./compiled",
                    "-l",
                    "iec61131std",
                    "-l",
                    "NX1P2",
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
