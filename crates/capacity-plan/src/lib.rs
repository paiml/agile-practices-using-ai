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

/// Reads `id,points` lines; `#` lines and an `id,` header are skipped.
pub fn parse(text: &str) -> Result<Vec<Estimate>, String> {
    let mut out = Vec::new();
    for (n, raw) in text.lines().enumerate() {
        let raw = raw.trim();
        if raw.is_empty() || raw.starts_with('#') || raw.starts_with("id,") {
            continue;
        }
        let (id, points) = raw
            .split_once(',')
            .ok_or(format!("line {}: expected id,points", n + 1))?;
        let points = points
            .trim()
            .parse()
            .map_err(|_| format!("line {}: points is not a whole number", n + 1))?;
        out.push(Estimate {
            id: id.trim().to_string(),
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
    fn sample_file_parses() {
        assert!(!parse(include_str!("../samples/estimates.csv"))
            .expect("parses")
            .is_empty());
    }
}
