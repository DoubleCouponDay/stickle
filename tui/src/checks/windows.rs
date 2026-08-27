use std::path::{Path, PathBuf};

use crate::builds::{OMRON_ARTIFACT, OMRON_CHECK};
use crate::env::EnvSnapshot;
use crate::probe::Probes;

use super::{Check, Group, Status, file_check, find_in, which};

pub const NAME: &str = "Windows";
pub const DOTNET_REMEDY: &str = "https://dotnet.microsoft.com/download";
pub const REQUIRED_LLVM: &str = "21.1.7";

const PLC_PIPELINE: &str =
    "https://github.com/doublecouponday/rusty-fork/actions/workflows/windows.yml";
const LLVM_RELEASE: &str =
    "https://github.com/PLC-lang/llvm-package-windows/releases/tag/v21.1.7";

pub fn candidates(dir: &Path, name: &str) -> Vec<PathBuf> {
    ["exe", "cmd", "bat"]
        .iter()
        .map(|extension| dir.join(format!("{name}.{extension}")))
        .collect()
}

pub fn groups(env: &EnvSnapshot, probes: &mut Probes, root: &Path) -> Vec<Group> {
    vec![
        toolchain_group(env, probes),
        lib_group(env),
        runtime_group(env),
        exports_group(root),
        artifact_group(root),
    ]
}

fn artifact_group(root: &Path) -> Group {
    let check = file_check(
        root,
        OMRON_ARTIFACT,
        OMRON_CHECK,
        "the library that the lib_structured_text builds link against with -l libNX1P2",
        vec![
            "lib_structured_text.dll and lib_structured_text.xml cannot be built until it exists."
                .into(),
            "It is produced from the libomron sources, so build libNX1P2.dll first:".into(),
            "plc ./libomron/*.st -c -l iec61131std -l ws2_32 -l ntdll -l userenv -o ./compiled/libNX1P2.o"
                .into(),
            "clang ./compiled/libNX1P2.o --shared -l iec61131std -l ws2_32 -l ntdll -l userenv -fuse-ld=lld-link \"-Wl,/DEF:libomron/exports.def\" -o ./compiled/libNX1P2.dll"
                .into(),
            "The Build pane runs both steps for you.".into(),
        ],
    );

    Group {
        title: "Build artifacts".into(),
        checks: vec![check],
    }
}

fn toolchain_group(env: &EnvSnapshot, probes: &mut Probes) -> Group {
    let mut checks = Vec::new();

    checks.push(match which(env, "plc") {
        Some(path) => {
            let version = probes
                .first_line(&path, &["--version"])
                .unwrap_or_else(|| "version not reported".into());

            Check {
                name: "plc (Rusty Compiler) on PATH".into(),
                status: Status::Pass,
                summary: version.clone(),
                expected: "plc.exe resolvable through PATH".into(),
                found: format!("{}\n{}", path.display(), version),
                remedy: Vec::new(),
            }
        }
        None => Check {
            name: "plc (Rusty Compiler) on PATH".into(),
            status: Status::Fail,
            summary: "not found on PATH".into(),
            expected: "plc.exe resolvable through PATH".into(),
            found: format!("no plc.exe in any of the {} PATH folder(s)", env.path.len()),
            remedy: vec![
                "Download plc.zip from the Rusty Compiler Windows build pipeline:".into(),
                PLC_PIPELINE.into(),
                "Extract it to a permanent location, an AppData folder is recommended:".into(),
                "%LOCALAPPDATA%\\rustycompiler".into(),
                "Add that folder to PATH.".into(),
            ],
        },
    });

    checks.push(match which(env, "clang") {
        Some(path) => {
            let line = probes.first_line(&path, &["--version"]).unwrap_or_default();

            match clang_version(&line).as_deref() {
                Some(REQUIRED_LLVM) => Check {
                    name: format!("clang {REQUIRED_LLVM} on PATH"),
                    status: Status::Pass,
                    summary: format!("clang {REQUIRED_LLVM}"),
                    expected: format!("clang from LLVM {REQUIRED_LLVM}"),
                    found: format!("{}\n{}", path.display(), line),
                    remedy: Vec::new(),
                },
                Some(other) => Check {
                    name: format!("clang {REQUIRED_LLVM} on PATH"),
                    status: Status::Warn,
                    summary: format!("clang {other}, expected {REQUIRED_LLVM}"),
                    expected: format!("clang from LLVM {REQUIRED_LLVM}"),
                    found: format!("{}\n{}", path.display(), line),
                    remedy: vec![
                        format!(
                            "clang {other} may emit objects the Rusty Compiler stdlib was not built against."
                        ),
                        format!("Install LLVM {REQUIRED_LLVM} and put its bin folder first on PATH:"),
                        LLVM_RELEASE.into(),
                    ],
                },
                None => Check {
                    name: format!("clang {REQUIRED_LLVM} on PATH"),
                    status: Status::Warn,
                    summary: "version could not be read".into(),
                    expected: format!("clang from LLVM {REQUIRED_LLVM}"),
                    found: format!("{}\n{line}", path.display()),
                    remedy: vec!["Run clang --version manually to inspect the install.".into()],
                },
            }
        }
        None => Check {
            name: format!("clang {REQUIRED_LLVM} on PATH"),
            status: Status::Fail,
            summary: "not found on PATH".into(),
            expected: format!("clang from LLVM {REQUIRED_LLVM}"),
            found: format!("no clang.exe in any of the {} PATH folder(s)", env.path.len()),
            remedy: vec![
                format!("Install LLVM {REQUIRED_LLVM}:"),
                LLVM_RELEASE.into(),
                "Add its bin folder to PATH.".into(),
            ],
        },
    });

    checks.push(match which(env, "lld-link") {
        Some(path) => Check {
            name: "lld-link on PATH".into(),
            status: Status::Pass,
            summary: "available".into(),
            expected: "lld-link.exe, used by clang through -fuse-ld=lld-link".into(),
            found: path.display().to_string(),
            remedy: Vec::new(),
        },
        None => Check {
            name: "lld-link on PATH".into(),
            status: Status::Fail,
            summary: "not found on PATH".into(),
            expected: "lld-link.exe, used by clang through -fuse-ld=lld-link".into(),
            found: format!(
                "no lld-link.exe in any of the {} PATH folder(s)",
                env.path.len()
            ),
            remedy: vec![
                "lld-link ships inside the LLVM bin folder.".into(),
                "Add the LLVM bin folder to PATH.".into(),
            ],
        },
    });

    Group {
        title: "Compiler toolchain".into(),
        checks,
    }
}

enum LibSource {
    Stdlib,
    WindowsSdkUm,
    WindowsSdkUcrt,
    Msvc,
}

impl LibSource {
    fn origin(&self) -> &'static str {
        match self {
            LibSource::Stdlib => "Rusty Compiler stdlib",
            LibSource::WindowsSdkUm => "Windows SDK, Lib\\<sdk version>\\um\\x64",
            LibSource::WindowsSdkUcrt => "Windows SDK, Lib\\<sdk version>\\ucrt\\x64",
            LibSource::Msvc => "MSVC build tools, VC\\Tools\\MSVC\\<version>\\lib\\x64",
        }
    }

    fn remedy(&self) -> Vec<String> {
        match self {
            LibSource::Stdlib => vec![
                "Download stdlib.lib from the Rusty Compiler Windows build pipeline:".into(),
                PLC_PIPELINE.into(),
                "Install it next to plc.exe as iec61131std.lib.".into(),
                "Add that folder to the LIB environment variable.".into(),
            ],
            LibSource::WindowsSdkUm | LibSource::WindowsSdkUcrt => vec![
                "Install the Windows SDK with the Visual Studio Installer, or standalone.".into(),
                "Add its x64 lib folders to the LIB environment variable:".into(),
                "C:\\Program Files (x86)\\Windows Kits\\10\\Lib\\<sdk version>\\um\\x64".into(),
                "C:\\Program Files (x86)\\Windows Kits\\10\\Lib\\<sdk version>\\ucrt\\x64".into(),
            ],
            LibSource::Msvc => vec![
                "Install MSVC with the Visual Studio Installer, or standalone.".into(),
                "Add its x64 lib folder to the LIB environment variable:".into(),
                "C:\\Program Files\\Microsoft Visual Studio\\<edition>\\VC\\Tools\\MSVC\\<version>\\lib\\x64"
                    .into(),
            ],
        }
    }
}

fn required_libs() -> Vec<(&'static str, LibSource)> {
    vec![
        ("iec61131std.lib", LibSource::Stdlib),
        ("ws2_32.lib", LibSource::WindowsSdkUm),
        ("ntdll.lib", LibSource::WindowsSdkUm),
        ("userenv.lib", LibSource::WindowsSdkUm),
        ("libcmt.lib", LibSource::Msvc),
        ("oldnames.lib", LibSource::Msvc),
        ("libucrt.lib", LibSource::WindowsSdkUcrt),
    ]
}

fn lib_group(env: &EnvSnapshot) -> Group {
    let libs = &env.lib;
    let mut checks = Vec::new();

    checks.push(if !env.lib_defined || libs.is_empty() {
        Check {
            name: "LIB environment variable".into(),
            status: Status::Fail,
            summary: "not set".into(),
            expected: "LIB holding the folders that contain every link library".into(),
            found: format!("LIB is empty or undefined, read from {}", env.source),
            remedy: vec![
                "Create a LIB environment variable containing these folders:".into(),
                "%LOCALAPPDATA%\\rustycompiler".into(),
                "C:\\Program Files (x86)\\Windows Kits\\10\\Lib\\<sdk version>\\um\\x64".into(),
                "C:\\Program Files (x86)\\Windows Kits\\10\\Lib\\<sdk version>\\ucrt\\x64".into(),
                "C:\\Program Files\\Microsoft Visual Studio\\<edition>\\VC\\Tools\\MSVC\\<version>\\lib\\x64"
                    .into(),
            ],
        }
    } else {
        let missing = libs.iter().filter(|dir| !dir.is_dir()).count();

        let listing = libs
            .iter()
            .map(|dir| {
                let mark = if dir.is_dir() { "ok" } else { "missing" };
                format!("[{mark}] {}", dir.display())
            })
            .collect::<Vec<_>>()
            .join("\n");

        Check {
            name: "LIB environment variable".into(),
            status: if missing == 0 {
                Status::Pass
            } else {
                Status::Warn
            },
            summary: format!("{} folder(s), {missing} missing from disk", libs.len()),
            expected: "LIB holding the folders that contain every link library".into(),
            found: listing,
            remedy: if missing == 0 {
                Vec::new()
            } else {
                vec![
                    "The folders marked missing do not exist on disk.".into(),
                    "Correct the paths in the LIB environment variable.".into(),
                ]
            },
        }
    });

    for (lib, source) in required_libs() {
        checks.push(match find_in(libs, lib) {
            Some(path) => Check {
                name: lib.into(),
                status: Status::Pass,
                summary: "found".into(),
                expected: format!(
                    "{lib} reachable through LIB, supplied by the {}",
                    source.origin()
                ),
                found: path.display().to_string(),
                remedy: Vec::new(),
            },
            None => Check {
                name: lib.into(),
                status: Status::Fail,
                summary: "not in any LIB folder".into(),
                expected: format!(
                    "{lib} reachable through LIB, supplied by the {}",
                    source.origin()
                ),
                found: format!(
                    "{lib} is not present in any of the {} LIB folder(s)",
                    libs.len()
                ),
                remedy: source.remedy(),
            },
        });
    }

    Group {
        title: "Link libraries (LIB)".into(),
        checks,
    }
}

fn runtime_group(env: &EnvSnapshot) -> Group {
    let check = match find_in(&env.path, "iec61131std.dll") {
        Some(path) => Check {
            name: "iec61131std.dll on PATH".into(),
            status: Status::Pass,
            summary: "resolvable".into(),
            expected: "iec61131std.dll loadable at run time by lib_structured_text.dll".into(),
            found: path.display().to_string(),
            remedy: Vec::new(),
        },
        None => Check {
            name: "iec61131std.dll on PATH".into(),
            status: Status::Fail,
            summary: "not found on PATH".into(),
            expected: "iec61131std.dll loadable at run time by lib_structured_text.dll".into(),
            found: format!(
                "no iec61131std.dll in any of the {} PATH folder(s)",
                env.path.len()
            ),
            remedy: vec![
                "The compiled library imports the stdlib at run time, so dotnet test cannot load it without this DLL."
                    .into(),
                "Take it from the Windows build pipeline artifacts and install it next to plc.exe:"
                    .into(),
                PLC_PIPELINE.into(),
                "Keep that folder on PATH.".into(),
            ],
        },
    };

    Group {
        title: "Run time libraries (PATH)".into(),
        checks: vec![check],
    }
}

fn exports_group(root: &Path) -> Group {
    let checks = vec![
        file_check(
            root,
            "exports.def",
            "exports.def",
            "the export list handed to lld-link for lib_structured_text.dll",
            vec!["Restore the file from source control.".into()],
        ),
        file_check(
            root,
            "libomron/exports.def",
            "libomron/exports.def",
            "the export list handed to lld-link for libNX1P2.dll",
            vec!["Restore the file from source control.".into()],
        ),
    ];

    Group {
        title: "Link exports".into(),
        checks,
    }
}

fn clang_version(line: &str) -> Option<String> {
    let mut tokens = line.split_whitespace();

    while let Some(token) = tokens.next() {
        if token == "version" {
            return tokens.next().map(str::to_string);
        }
    }

    None
}
