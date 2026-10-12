//! One gate that says Done: run every check, and pass only when all of them
//! pass. An empty gate is not Done, and a red check is never reported green.
//!
//! The same binary runs in a git hook, in CI, and when an AI assistant claims
//! a task is finished. Contract: `contracts/done-gate-v1.yaml`.

use std::fmt;
use std::process::{Command, Stdio};

/// One check: a name and a program with its arguments (no shell).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Check {
    pub name: String,
    pub program: String,
    pub args: Vec<String>,
}

/// The gate's answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// Every check passed, and there was at least one.
    Done,
    /// These checks failed or could not start.
    NotDone(Vec<String>),
    /// There were no checks, so nothing was shown to be Done.
    Empty,
}

impl Verdict {
    /// Process exit code: 0 Done, 1 NotDone, 2 Empty.
    pub fn code(&self) -> u8 {
        match self {
            Self::Done => 0,
            Self::NotDone(_) => 1,
            Self::Empty => 2,
        }
    }
}

impl fmt::Display for Verdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Done => write!(f, "DONE"),
            Self::NotDone(red) => write!(f, "NOT DONE: {}", red.join(", ")),
            Self::Empty => write!(f, "NOT DONE: the gate has no checks"),
        }
    }
}

/// Contract equation `done_iff_all_green`: Done exactly when the list is
/// non-empty and every result is a pass.
pub fn verdict(results: &[(String, bool)]) -> Verdict {
    if results.is_empty() {
        return Verdict::Empty;
    }
    let red: Vec<String> = results
        .iter()
        .filter(|(_, ok)| !ok)
        .map(|(n, _)| n.clone())
        .collect();
    if red.is_empty() {
        Verdict::Done
    } else {
        Verdict::NotDone(red)
    }
}

/// Runs one check. A program that cannot start counts as a failure.
pub fn run(check: &Check) -> bool {
    Command::new(&check.program)
        .args(&check.args)
        .stdin(Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

/// Reads `name: program arg...` lines; `#` lines and blank lines are skipped.
/// Arguments are split on whitespace; there is no shell and no quoting.
pub fn parse(text: &str) -> Result<Vec<Check>, String> {
    let mut out = Vec::new();
    for (n, raw) in text.lines().enumerate() {
        let raw = raw.trim();
        if raw.is_empty() || raw.starts_with('#') {
            continue;
        }
        let (name, cmd) = raw
            .split_once(':')
            .ok_or(format!("line {}: expected `name: program args`", n + 1))?;
        let mut words = cmd.split_whitespace().map(str::to_string);
        let program = words.next().ok_or(format!("line {}: no program", n + 1))?;
        out.push(Check {
            name: name.trim().to_string(),
            program,
            args: words.collect(),
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(name: &str, ok: bool) -> (String, bool) {
        (name.to_string(), ok)
    }

    #[test]
    fn all_green_is_done() {
        assert_eq!(verdict(&[r("fmt", true), r("test", true)]), Verdict::Done);
    }

    /// FALSIFY-DONE-001: one red check makes the gate not Done.
    #[test]
    fn falsify_one_red_check_is_not_done() {
        let v = verdict(&[r("fmt", true), r("test", false)]);
        assert_eq!(v, Verdict::NotDone(vec!["test".into()]));
        assert_ne!(v.code(), 0);
    }

    /// FALSIFY-DONE-002: an empty gate is not Done.
    #[test]
    fn falsify_empty_gate_is_not_done() {
        assert_eq!(verdict(&[]), Verdict::Empty);
        assert_ne!(verdict(&[]).code(), 0);
    }

    /// FALSIFY-DONE-003: a check whose program does not exist counts as red.
    #[test]
    fn falsify_missing_program_is_red() {
        let check = Check {
            name: "ghost".into(),
            program: "no-such-program-on-path".into(),
            args: vec![],
        };
        assert!(!run(&check));
    }

    #[test]
    fn sample_checks_parse() {
        let checks = parse(include_str!("../samples/checks.txt")).expect("parses");
        assert!(checks.iter().any(|c| c.program == "cargo"));
    }
}
