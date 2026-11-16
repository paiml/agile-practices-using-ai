//! `done-gate <checks.txt>`: runs every check in order and prints the
//! verdict. Exit 0 Done, 1 a check failed, 2 no checks or a usage error.

use std::process::ExitCode;

fn main() -> ExitCode {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: done-gate <checks.txt>");
        return ExitCode::from(2);
    };
    let checks = match std::fs::read_to_string(&path)
        .map_err(|e| e.to_string())
        .and_then(|t| done_gate::parse(&t))
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{path}: {e}");
            return ExitCode::from(2);
        }
    };
    let mut results = Vec::new();
    for check in &checks {
        let ok = done_gate::run(check);
        println!("{} {}", if ok { "pass" } else { "FAIL" }, check.name);
        results.push((check.name.clone(), ok));
    }
    let verdict = done_gate::verdict(&results);
    println!("{verdict}");
    ExitCode::from(verdict.code())
}
