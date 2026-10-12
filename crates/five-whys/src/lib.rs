//! A retrospective root-cause chain as data: a problem, at least five whys,
//! each one new, and an action someone can take.
//!
//! An AI assistant can draft the chain from the retro notes; this crate
//! refuses a chain that stops early, goes in a circle, or ends without an
//! action. Contract: `contracts/five-whys-v1.yaml`.

use std::fmt;

/// The minimum number of whys.
pub const MIN_DEPTH: usize = 5;

/// A root-cause chain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chain {
    pub problem: String,
    pub whys: Vec<String>,
    pub action: String,
}

/// Why a chain was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChainError {
    TooShallow { depth: usize },
    Repeated { index: usize },
    NoAction,
    NoProblem,
}

impl fmt::Display for ChainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooShallow { depth } => {
                write!(f, "only {depth} whys; at least {MIN_DEPTH} are needed")
            }
            Self::Repeated { index } => {
                write!(f, "why {} repeats an earlier answer (a cycle)", index + 1)
            }
            Self::NoAction => write!(f, "the chain ends without an action"),
            Self::NoProblem => write!(f, "the chain has no problem statement"),
        }
    }
}

impl std::error::Error for ChainError {}

/// Contract equation `chain_well_formed`: a problem, at least [`MIN_DEPTH`]
/// whys with no answer repeated (compared ignoring case), and an action.
pub fn validate(chain: &Chain) -> Result<(), ChainError> {
    if chain.problem.trim().is_empty() {
        return Err(ChainError::NoProblem);
    }
    if chain.whys.len() < MIN_DEPTH {
        return Err(ChainError::TooShallow {
            depth: chain.whys.len(),
        });
    }
    for (index, why) in chain.whys.iter().enumerate() {
        let key = why.trim().to_lowercase();
        if key.is_empty()
            || chain.whys[..index]
                .iter()
                .any(|w| w.trim().to_lowercase() == key)
        {
            return Err(ChainError::Repeated { index });
        }
    }
    if chain.action.trim().is_empty() {
        return Err(ChainError::NoAction);
    }
    Ok(())
}

/// Reads `Problem: ...`, `Why: ...` (one per line) and `Action: ...` lines.
pub fn parse(text: &str) -> Result<Chain, String> {
    let mut chain = Chain {
        problem: String::new(),
        whys: Vec::new(),
        action: String::new(),
    };
    for (n, raw) in text.lines().enumerate() {
        let raw = raw.trim();
        if raw.is_empty() || raw.starts_with('#') {
            continue;
        }
        let (key, value) = raw
            .split_once(':')
            .ok_or(format!("line {}: expected `Key: text`", n + 1))?;
        let value = value.trim().to_string();
        match key.trim() {
            "Problem" => chain.problem = value,
            "Why" => chain.whys.push(value),
            "Action" => chain.action = value,
            other => return Err(format!("line {}: unknown key `{other}`", n + 1)),
        }
    }
    Ok(chain)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chain(whys: &[&str], action: &str) -> Chain {
        Chain {
            problem: "the release slipped".into(),
            whys: whys.iter().map(|w| w.to_string()).collect(),
            action: action.into(),
        }
    }

    const FIVE: [&str; 5] = ["a", "b", "c", "d", "e"];

    #[test]
    fn five_distinct_whys_and_an_action_pass() {
        assert_eq!(validate(&chain(&FIVE, "add a check")), Ok(()));
    }

    /// FALSIFY-WHY-001: four whys are not enough.
    #[test]
    fn falsify_depth_four_is_refused() {
        assert_eq!(
            validate(&chain(&FIVE[..4], "x")),
            Err(ChainError::TooShallow { depth: 4 })
        );
    }

    /// FALSIFY-WHY-002: a repeated why is a cycle and is refused.
    #[test]
    fn falsify_cycle_is_refused() {
        assert_eq!(
            validate(&chain(&["a", "b", "c", "B", "e"], "x")),
            Err(ChainError::Repeated { index: 3 })
        );
    }

    /// FALSIFY-WHY-003: a chain without an action is refused.
    #[test]
    fn falsify_missing_action_is_refused() {
        assert_eq!(validate(&chain(&FIVE, " ")), Err(ChainError::NoAction));
    }

    #[test]
    fn sample_chain_is_well_formed() {
        let c = parse(include_str!("../samples/retro.txt")).expect("parses");
        assert_eq!(validate(&c), Ok(()));
    }
}
