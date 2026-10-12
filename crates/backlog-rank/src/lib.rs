//! Deterministic backlog ranking: value per unit of effort, risk as the
//! tiebreak, id as the last word.
//!
//! An AI assistant can propose scores; the ranking itself should be a pure
//! function, so the same backlog always gives the same order and nothing is
//! lost or duplicated on the way. Contract: `contracts/backlog-rank-v1.yaml`.

use std::cmp::Ordering;
use std::fmt;

/// One backlog item. `effort` is never zero (see [`parse`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub id: String,
    pub value: u32,
    pub effort: u32,
    pub risk: u32,
}

/// Why a backlog file was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub line: usize,
    pub reason: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.reason)
    }
}

impl std::error::Error for ParseError {}

/// Total order used by [`rank`]: higher value/effort first (compared by cross
/// multiplication, so there is no rounding), then higher risk, then id.
pub fn compare(a: &Item, b: &Item) -> Ordering {
    let lhs = u64::from(b.value) * u64::from(a.effort);
    let rhs = u64::from(a.value) * u64::from(b.effort);
    lhs.cmp(&rhs)
        .then_with(|| b.risk.cmp(&a.risk))
        .then_with(|| a.id.cmp(&b.id))
}

/// Contract equation `rank_total_order`: returns a permutation of `items`
/// sorted by [`compare`].
pub fn rank(items: &[Item]) -> Vec<Item> {
    let mut out = items.to_vec();
    out.sort_by(compare);
    out
}

/// Reads `id,value,effort,risk` lines. A header line starting with `id` and
/// lines starting with `#` are skipped. Zero effort and duplicate ids are refused.
pub fn parse(text: &str) -> Result<Vec<Item>, ParseError> {
    let mut items: Vec<Item> = Vec::new();
    for (n, raw) in text.lines().enumerate() {
        let line = n + 1;
        let raw = raw.trim();
        if raw.is_empty() || raw.starts_with('#') || raw.starts_with("id,") {
            continue;
        }
        let err = |reason: &str| ParseError {
            line,
            reason: reason.to_string(),
        };
        let f: Vec<&str> = raw.split(',').map(str::trim).collect();
        let [id, value, effort, risk] = f.as_slice() else {
            return Err(err("expected 4 fields: id,value,effort,risk"));
        };
        let num = |s: &str| s.parse::<u32>().map_err(|_| err("not a whole number"));
        let item = Item {
            id: (*id).to_string(),
            value: num(value)?,
            effort: num(effort)?,
            risk: num(risk)?,
        };
        if item.effort == 0 {
            return Err(err("effort must be at least 1"));
        }
        if items.iter().any(|i| i.id == item.id) {
            return Err(err("duplicate id"));
        }
        items.push(item);
    }
    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(id: &str, value: u32, effort: u32, risk: u32) -> Item {
        Item {
            id: id.into(),
            value,
            effort,
            risk,
        }
    }

    fn sample() -> Vec<Item> {
        vec![
            item("C", 3, 3, 1),
            item("A", 8, 2, 0),
            item("B", 4, 1, 0),
            item("D", 2, 2, 5),
        ]
    }

    #[test]
    fn ranks_by_value_per_effort_then_risk_then_id() {
        let ids: Vec<String> = rank(&sample()).into_iter().map(|i| i.id).collect();
        // A and B both score 4; D (risk 5) beats C (risk 1) at score 1.
        assert_eq!(ids, ["A", "B", "D", "C"]);
    }

    /// FALSIFY-RANK-001: the output is a permutation of the input.
    #[test]
    fn falsify_rank_is_a_permutation() {
        let input = sample();
        let mut a: Vec<String> = input.iter().map(|i| i.id.clone()).collect();
        let mut b: Vec<String> = rank(&input).into_iter().map(|i| i.id).collect();
        a.sort();
        b.sort();
        assert_eq!(a, b);
    }

    /// FALSIFY-RANK-002: input order does not change the result.
    #[test]
    fn falsify_rank_is_order_independent() {
        let mut reversed = sample();
        reversed.reverse();
        assert_eq!(rank(&sample()), rank(&reversed));
    }

    /// FALSIFY-RANK-003: zero effort is refused rather than divided by.
    #[test]
    fn falsify_zero_effort_is_refused() {
        assert_eq!(parse("X,5,0,1").map_err(|e| e.line), Err(1));
    }

    #[test]
    fn duplicate_id_is_refused() {
        assert!(parse("X,1,1,1\nX,2,1,1").is_err());
    }

    #[test]
    fn sample_file_parses() {
        let items = parse(include_str!("../samples/backlog.csv")).expect("sample parses");
        assert_eq!(rank(&items).len(), items.len());
    }
}
