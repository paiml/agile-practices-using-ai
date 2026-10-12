//! `working-agreement <CLAUDE.md>`: exit 0 when every required section is
//! present and non-empty, 1 when one is missing, 2 on a usage or read error.

use std::process::ExitCode;

fn main() -> ExitCode {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: working-agreement <CLAUDE.md>");
        return ExitCode::from(2);
    };
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{path}: {e}");
            return ExitCode::from(2);
        }
    };
    let missing = working_agreement::missing_sections(&text);
    if missing.is_empty() {
        println!("{path}: ok");
        return ExitCode::SUCCESS;
    }
    for name in missing {
        println!("{path}: missing or empty section `## {name}`");
    }
    ExitCode::from(1)
}
