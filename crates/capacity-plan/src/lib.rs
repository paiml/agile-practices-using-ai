//! Sprint planning against capacity: no sprint is overfilled, and no item is
//! dropped. Whatever does not fit goes to the overflow list.
//!
//! An AI assistant can estimate points; the plan built from those estimates
//! must still respect capacity and conserve work. Contract:
//! `contracts/capacity-plan-v1.yaml`.

/// A backlog item with its estimate in points.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Estimate {
    pub id: String,
    pub points: u32,
}

/// The result of planning: one list of ids per sprint, plus what did not fit.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Plan {
    pub sprints: Vec<Vec<String>>,
    pub overflow: Vec<String>,
}

/// Fills sprints in backlog order. An item goes into the first sprint with
/// room for it; an item that fits no sprint goes to `overflow`.
///
/// Contract equations `capacity_respected` (each sprint's points ≤ capacity)
/// and `work_conserved` (every id appears exactly once in sprints ∪ overflow).
pub fn plan(items: &[Estimate], capacity: u32, sprints: usize) -> Plan {
    let mut out = Plan {
        sprints: vec![Vec::new(); sprints],
        overflow: Vec::new(),
    };
    let mut used = vec![0u32; sprints];
    for item in items {
        let slot = used
            .iter()
            .position(|&u| u.checked_add(item.points).is_some_and(|t| t <= capacity));
        match slot {
            Some(s) => {
                used[s] += item.points;
                out.sprints[s].push(item.id.clone());
            }
            None => out.overflow.push(item.id.clone()),
        }
    }
    out
}

/// Points planned into each sprint.
pub fn load(items: &[Estimate], plan: &Plan) -> Vec<u32> {
    plan.sprints
        .iter()
        .map(|ids| {
            ids.iter()
                .filter_map(|id| items.iter().find(|i| &i.id == id))
                .map(|i| i.points)
                .sum()
        })
        .collect()
}

/// Reads `id,points` lines; `#` lines and the exact header `id,points` are skipped.
/// A duplicate id is refused, so no item can be planned twice.
pub fn parse(text: &str) -> Result<Vec<Estimate>, String> {
    let mut out = Vec::new();
    for (n, raw) in text.lines().enumerate() {
        let raw = raw.trim();
        if raw.is_empty() || raw.starts_with('#') || raw.replace(' ', "") == "id,points" {
            continue;
        }
        let (id, points) = raw
            .split_once(',')
            .ok_or(format!("line {}: expected id,points", n + 1))?;
        let points = points
            .trim()
            .parse()
            .map_err(|_| format!("line {}: points is not a whole number", n + 1))?;
        let id = id.trim();
        if out.iter().any(|e: &Estimate| e.id == id) {
            return Err(format!("line {}: duplicate id `{id}`", n + 1));
        }
        out.push(Estimate {
            id: id.to_string(),
            points,
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn est(id: &str, points: u32) -> Estimate {
        Estimate {
            id: id.into(),
            points,
        }
    }

    fn backlog() -> Vec<Estimate> {
        vec![
            est("A", 5),
            est("B", 8),
            est("C", 3),
            est("D", 13),
            est("E", 2),
            est("F", 21),
        ]
    }

    /// FALSIFY-CAP-001: no sprint is planned over capacity.
    #[test]
    fn falsify_no_sprint_over_capacity() {
        let items = backlog();
        let p = plan(&items, 13, 2);
        assert!(
            load(&items, &p).iter().all(|&l| l <= 13),
            "{:?}",
            load(&items, &p)
        );
    }

    /// FALSIFY-CAP-002: every item is planned or in overflow, exactly once.
    #[test]
    fn falsify_work_is_conserved() {
        let items = backlog();
        let p = plan(&items, 13, 2);
        let mut seen: Vec<String> = p.sprints.concat();
        seen.extend(p.overflow.clone());
        seen.sort();
        let mut want: Vec<String> = items.iter().map(|i| i.id.clone()).collect();
        want.sort();
        assert_eq!(seen, want);
    }

    /// FALSIFY-CAP-005: one point over capacity overflows; an exact fill is planned.
    #[test]
    fn falsify_capacity_boundary_is_exact() {
        let over = plan(&[est("OVER", 14)], 13, 1);
        assert_eq!(over.overflow, ["OVER"]);
        let exact = plan(&[est("A", 8), est("B", 5)], 13, 1);
        assert_eq!(exact.sprints[0], ["A", "B"]);
        assert!(exact.overflow.is_empty());
    }

    /// FALSIFY-CAP-006: a duplicate id is refused, so it cannot be planned twice.
    #[test]
    fn falsify_duplicate_id_is_refused() {
        assert!(parse("A,5\nA,8").is_err());
    }

    #[test]
    fn oversized_item_overflows() {
        let p = plan(&[est("BIG", 40)], 20, 3);
        assert_eq!(p.overflow, ["BIG"]);
    }

    #[test]
    fn zero_sprints_overflow_everything() {
        let p = plan(&backlog(), 13, 0);
        assert_eq!(p.overflow.len(), backlog().len());
    }

    #[test]
    fn an_item_named_id_is_read() {
        assert_eq!(parse("id,3").map(|v| v.len()), Ok(1));
    }

    #[test]
    fn sample_file_parses() {
        assert!(!parse(include_str!("../samples/estimates.csv"))
            .expect("parses")
            .is_empty());
    }
}
