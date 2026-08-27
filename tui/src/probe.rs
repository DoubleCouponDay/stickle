use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::SystemTime;

type Stamp = Option<(u64, Option<SystemTime>)>;

struct Entry {
    stamp: Stamp,
    output: String,
}

pub struct Probes {
    cache: HashMap<String, Entry>,
}

impl Probes {
    pub fn new() -> Self {
        Probes {
            cache: HashMap::new(),
        }
    }

    pub fn run(&mut self, program: &Path, args: &[&str]) -> String {
        let key = format!("{}|{}", program.display(), args.join(" "));
        let stamp = stamp(program);

        if let Some(entry) = self.cache.get(&key) {
            if entry.stamp == stamp {
                return entry.output.clone();
            }
        }

        let output = capture(program, args);

        self.cache.insert(
            key,
            Entry {
                stamp,
                output: output.clone(),
            },
        );

        output
    }

    pub fn first_line(&mut self, program: &Path, args: &[&str]) -> Option<String> {
        let output = self.run(program, args);

        output
            .lines()
            .map(str::trim)
            .find(|line| !line.is_empty())
            .map(str::to_string)
    }

    pub fn lines(&mut self, program: &Path, args: &[&str]) -> Vec<String> {
        self.run(program, args)
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::to_string)
            .collect()
    }
}

fn capture(program: &Path, args: &[&str]) -> String {
    let Ok(result) = Command::new(program).args(args).output() else {
        return String::new();
    };

    let stdout = String::from_utf8_lossy(&result.stdout).to_string();

    if stdout.trim().is_empty() {
        return String::from_utf8_lossy(&result.stderr).to_string();
    }

    stdout
}

fn stamp(program: &Path) -> Stamp {
    let metadata = fs::metadata(program).ok()?;

    Some((metadata.len(), metadata.modified().ok()))
}
