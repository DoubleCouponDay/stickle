use std::env;
use std::path::{Path, PathBuf};

use crate::builds::{OMRON_ARTIFACT, OMRON_CHECK};
use crate::env::EnvSnapshot;
use crate::probe::Probes;

use super::{Check, Group, Status, file_check, find_in, which};

pub const NAME: &str = "Linux";
pub const DOTNET_REMEDY: &str =
    "sudo snap install --classic dotnet && sudo snap install dotnet-sdk-100";

const PLC_PIPELINE: &str =
    "https://github.com/doublecouponday/rusty-fork/actions/workflows/linux.yml";

pub fn candidates(dir: &Path, name: &str) -> Vec<PathBuf> {
    vec![dir.join(name)]
}

pub fn groups(env: &EnvSnapshot, probes: &mut Probes, root: &Path) -> Vec<Group> {
    vec![
        toolchain_group(env, probes),
        shared_object_group(env),
        artifact_group(root),
    ]
}

fn artifact_group(root: &Path) -> Group {
    let check = file_check(
        root,
        OMRON_ARTIFACT,
        OMRON_CHECK,
        "the library that the lib_structured_text builds link against with -l NX1P2",
        vec![
            "lib_structured_text.so and lib_structured_text.xml cannot be built until it exists."
                .into(),
            "It is produced from the libomron sources, so build libNX1P2.so first:".into(),
            "plc ./libomron/*.st --shared --linker=cc --target=x86_64 -l iec61131std -o ./compiled/libNX1P2.so"
                .into(),
            "The Build pane runs that for you.".into(),
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
                expected: "plc resolvable through PATH, /usr/bin is the documented location".into(),
                found: format!("{}\n{}", path.display(), version),
                remedy: Vec::new(),
            }
        }
        None => Check {
            name: "plc (Rusty Compiler) on PATH".into(),
            status: Status::Fail,
            summary: "not found on PATH".into(),
            expected: "plc resolvable through PATH, /usr/bin is the documented location".into(),
            found: format!("no plc in any of the {} PATH folder(s)", env.path.len()),
            remedy: vec![
                "Download plc.zip from the Rusty Compiler Linux build pipeline:".into(),
                PLC_PIPELINE.into(),
                "Decompress it and install the executable:".into(),
                "7z e ./Downloads/plc.zip && sudo cp ./Downloads/plc /usr/bin".into(),
            ],
        },
    });

    checks.push(match which(env, "cc") {
        Some(path) => {
            let version = probes
                .first_line(&path, &["--version"])
                .unwrap_or_else(|| "version not reported".into());

            Check {
                name: "cc on PATH".into(),
                status: Status::Pass,
                summary: version.clone(),
                expected: "cc, the linker plc drives through --linker=cc".into(),
                found: format!("{}\n{}", path.display(), version),
                remedy: Vec::new(),
            }
        }
        None => Check {
            name: "cc on PATH".into(),
            status: Status::Fail,
            summary: "not found on PATH".into(),
            expected: "cc, the linker plc drives through --linker=cc".into(),
            found: format!("no cc in any of the {} PATH folder(s)", env.path.len()),
            remedy: vec![
                "cc comes from build-essential:".into(),
                "sudo apt update && sudo apt install build-essential".into(),
            ],
        },
    });

    Group {
        title: "Compiler toolchain".into(),
        checks,
    }
}

fn loader_dirs(env: &EnvSnapshot) -> Vec<PathBuf> {
    let triple = format!("{}-linux-gnu", env::consts::ARCH);

    let mut dirs = env.library_path.clone();

    dirs.extend([
        PathBuf::from("/lib"),
        PathBuf::from("/usr/lib"),
        PathBuf::from("/lib").join(&triple),
        PathBuf::from("/usr/lib").join(&triple),
        PathBuf::from("/usr/local/lib"),
    ]);

    dirs
}

fn shared_object_group(env: &EnvSnapshot) -> Group {
    let dirs = loader_dirs(env);
    let listing = dirs
        .iter()
        .map(|dir| dir.display().to_string())
        .collect::<Vec<_>>()
        .join("\n");

    let mut checks = Vec::new();

    checks.push(match find_in(&dirs, "libiec61131std.so") {
        Some(path) => Check {
            name: "libiec61131std.so".into(),
            status: Status::Pass,
            summary: "found".into(),
            expected: "libiec61131std.so on the loader search path, linked with -l iec61131std"
                .into(),
            found: path.display().to_string(),
            remedy: Vec::new(),
        },
        None => Check {
            name: "libiec61131std.so".into(),
            status: Status::Fail,
            summary: "not on the loader search path".into(),
            expected: "libiec61131std.so on the loader search path, linked with -l iec61131std"
                .into(),
            found: format!("searched:\n{listing}"),
            remedy: vec![
                "Download stdlib.zip from the Rusty Compiler Linux build pipeline:".into(),
                PLC_PIPELINE.into(),
                format!(
                    "Take the copy for your architecture, most likely {}-linux-gnu:",
                    env::consts::ARCH
                ),
                format!(
                    "sudo cp ~/Downloads/stdlib/{}-linux-gnu/libiec61131std.so /lib",
                    env::consts::ARCH
                ),
            ],
        },
    });

    checks.push(match find_in(&dirs, "libNX1P2.so") {
        Some(path) => Check {
            name: "libNX1P2.so".into(),
            status: Status::Pass,
            summary: "found".into(),
            expected: "libNX1P2.so on the loader search path, lib_structured_text.so records a dependency on it"
                .into(),
            found: path.display().to_string(),
            remedy: Vec::new(),
        },
        None => Check {
            name: "libNX1P2.so".into(),
            status: Status::Fail,
            summary: "not on the loader search path".into(),
            expected: "libNX1P2.so on the loader search path, lib_structured_text.so records a dependency on it"
                .into(),
            found: format!("searched:\n{listing}"),
            remedy: vec![
                "It is built from the libomron sources before the main sources are compiled:".into(),
                "plc ./libomron/*.st --shared --linker=cc --target=x86_64 -l iec61131std -o ./compiled/libNX1P2.so"
                    .into(),
                "Install it where the dynamic loader can find it:".into(),
                "sudo cp ./compiled/libNX1P2.so /lib".into(),
            ],
        },
    });

    Group {
        title: "Run time libraries (loader path)".into(),
        checks,
    }
}
