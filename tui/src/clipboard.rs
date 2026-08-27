use std::io::{self, Write};
use std::process::{Command, Stdio};

const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn copy(text: &str) -> Result<(), String> {
    let Err(native) = helper(text) else {
        return Ok(());
    };

    match osc52(text) {
        Ok(()) => Ok(()),
        Err(fallback) => Err(format!("{native}, {fallback}")),
    }
}

#[cfg(windows)]
fn helper(text: &str) -> Result<(), String> {
    let owned = text.replace('\n', "\r\n");
    let mut error = String::from("no clipboard helper");

    for program in ["clip.exe", "clip"] {
        match pipe(program, &[], &owned) {
            Ok(()) => return Ok(()),
            Err(reason) => error = reason,
        }
    }

    Err(error)
}

#[cfg(not(windows))]
fn helper(text: &str) -> Result<(), String> {
    let helpers: [(&str, &[&str]); 4] = [
        ("wl-copy", &[]),
        ("xclip", &["-selection", "clipboard"]),
        ("xsel", &["--clipboard", "--input"]),
        ("pbcopy", &[]),
    ];

    let mut error = String::from("no clipboard helper");

    for (program, args) in helpers {
        match pipe(program, args, text) {
            Ok(()) => return Ok(()),
            Err(reason) => error = reason,
        }
    }

    Err(error)
}

fn pipe(program: &str, args: &[&str], text: &str) -> Result<(), String> {
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| format!("{program}: {error}"))?;

    let Some(mut stdin) = child.stdin.take() else {
        return Err(format!("{program}: no stdin"));
    };

    stdin
        .write_all(text.as_bytes())
        .map_err(|error| format!("{program}: {error}"))?;

    drop(stdin);

    let status = child
        .wait()
        .map_err(|error| format!("{program}: {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("{program}: exited with {status}"))
    }
}

fn osc52(text: &str) -> Result<(), String> {
    let mut out = io::stdout().lock();

    write!(out, "\x1b]52;c;{}\x07", base64(text.as_bytes()))
        .map_err(|error| format!("OSC 52: {error}"))?;

    out.flush().map_err(|error| format!("OSC 52: {error}"))
}

fn base64(bytes: &[u8]) -> String {
    let mut encoded = String::new();

    for chunk in bytes.chunks(3) {
        let mut block = [0u8; 3];
        block[..chunk.len()].copy_from_slice(chunk);

        let packed = u32::from(block[0]) << 16 | u32::from(block[1]) << 8 | u32::from(block[2]);

        for index in 0..4 {
            if index <= chunk.len() {
                let shift = 18 - index * 6;
                encoded.push(ALPHABET[(packed >> shift & 0x3f) as usize] as char);
            } else {
                encoded.push('=');
            }
        }
    }

    encoded
}
