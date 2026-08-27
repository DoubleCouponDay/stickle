use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
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

#[cfg(windows)]
pub const OMRON_CHECK: &str = "libNX1P2.dll";
#[cfg(windows)]
pub const OMRON_ARTIFACT: &str = "compiled/libNX1P2.dll";

#[cfg(not(windows))]
pub const OMRON_CHECK: &str = "libNX1P2.so";
#[cfg(not(windows))]
pub const OMRON_ARTIFACT: &str = "compiled/libNX1P2.so";

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
    sender: Sender<Outcome>,
    pub running: Option<usize>,
}

impl Runner {
    pub fn new() -> Self {
        let (sender, outcomes) = mpsc::channel();

        Runner {
            outcomes,
            sender,
            running: None,
        }
    }

    pub fn start(&mut self, index: usize, target: &Target, root: PathBuf, snapshot: EnvSnapshot) {
        let label = target.label;
        let steps = target.steps.clone();
        let sender = self.sender.clone();

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
                    break;
                };

                let mut command = Command::new(&program);
                command.args(&step.args).current_dir(&root);

                if let Some(value) = join(&snapshot.path) {
                    command.env("PATH", value);
                }

                if let Some(value) = join(&snapshot.lib) {
                    command.env("LIB", value);
                }

                match command.output() {
                    Ok(output) => {
                        if let Some(line) = last_line(&output) {
                            message = line;
                        }

                        if !output.status.success() {
                            ok = false;
                            code = output.status.code();

                            if message.is_empty() {
                                message = format!("{} failed", step.program);
                            }

                            break;
                        }
                    }
                    Err(error) => {
                        ok = false;
                        message = format!("{}: {error}", step.program);
                        break;
                    }
                }
            }

            if ok && message.is_empty() {
                message = format!("{label} written to compiled");
            }

            let _ = sender.send(Outcome {
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
}

fn join(dirs: &[PathBuf]) -> Option<OsString> {
    if dirs.is_empty() {
        return None;
    }

    env::join_paths(dirs).ok()
}

fn last_line(output: &Output) -> Option<String> {
    let mut text = String::from_utf8_lossy(&output.stderr).to_string();

    if text.trim().is_empty() {
        text = String::from_utf8_lossy(&output.stdout).to_string();
    }

    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .next_back()
        .map(str::to_string)
}

#[cfg(windows)]
pub fn targets() -> Vec<Target> {
    vec![
        Target {
            label: "libNX1P2.dll",
            ignores: &[SOURCE_CHECK, OMRON_CHECK],
            requires: None,
            steps: vec![
                Step {
                    program: "plc",
                    args: args(&[
                        "./libomron/*.st",
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
                        "-Wl,/DEF:libomron/exports.def",
                        "-o",
                        "./compiled/libNX1P2.dll",
                    ]),
                },
            ],
        },
        Target {
            label: "lib_structured_text.dll",
            ignores: &[],
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
            ignores: &[],
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
    ]
}

#[cfg(not(windows))]
pub fn targets() -> Vec<Target> {
    vec![
        Target {
            label: "libNX1P2.so",
            ignores: &[SOURCE_CHECK, OMRON_CHECK],
            requires: None,
            steps: vec![Step {
                program: "plc",
                args: args(&[
                    "./libomron/*.st",
                    "--shared",
                    "--linker=cc",
                    "--target=x86_64",
                    "-l",
                    "iec61131std",
                    "-o",
                    "./compiled/libNX1P2.so",
                ]),
            }],
        },
        Target {
            label: "lib_structured_text.so",
            ignores: &[],
            requires: Some(OMRON_ARTIFACT),
            steps: vec![Step {
                program: "plc",
                args: args(&[
                    "./source/*.st",
                    "--shared",
                    "--linker=cc",
                    "--target=x86_64",
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
                    "./compiled/lib_structured_text.so",
                ]),
            }],
        },
        Target {
            label: "lib_structured_text.xml",
            ignores: &[],
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
    ]
}

fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}
