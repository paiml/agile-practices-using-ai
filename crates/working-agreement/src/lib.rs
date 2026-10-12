//! Lints a team working agreement written for an AI assistant (a `CLAUDE.md`).
//!
//! The agreement is the assistant's memory of how the team works. It is
//! complete only when it says what Done means, how branches work and how
//! review happens. Contract: `contracts/working-agreement-v1.yaml`.

/// The sections every agreement must have, each with a non-blank body.
pub const REQUIRED: [&str; 3] = ["Definition of Done", "Branching", "Review"];

/// Contract equation `agreement_complete`: returns the required sections that
/// are missing or empty. An empty result means the agreement is complete.
pub fn missing_sections(text: &str) -> Vec<&'static str> {
    REQUIRED
        .iter()
        .copied()
        .filter(|name| !has_section(text, name))
        .collect()
}

/// True when `## <name>` is a heading, and every heading with that name has at
/// least one non-blank line before the next heading of level 1 or 2.
fn has_section(text: &str, name: &str) -> bool {
    let heading = format!("## {name}");
    let lines: Vec<&str> = text.lines().collect();
    let starts: Vec<usize> = (0..lines.len())
        .filter(|&i| lines[i].trim_end() == heading)
        .collect();
    !starts.is_empty()
        && starts.iter().all(|&i| {
            lines[i + 1..]
                .iter()
                .take_while(|l| !(l.starts_with("# ") || l.starts_with("## ")))
                .any(|l| !l.trim().is_empty())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMPLETE: &str = "# Team\n## Definition of Done\n- tests pass\n## Branching\n- feature branches\n## Review\n- one reviewer\n";

    #[test]
    fn complete_agreement_passes() {
        assert!(missing_sections(COMPLETE).is_empty());
    }

    /// FALSIFY-WA-001: an agreement without a Definition of Done fails.
    #[test]
    fn falsify_missing_definition_of_done_fails() {
        let text = COMPLETE.replace("## Definition of Done\n- tests pass\n", "");
        assert_eq!(missing_sections(&text), ["Definition of Done"]);
    }

    /// FALSIFY-WA-002: a heading with no body does not count.
    #[test]
    fn falsify_empty_section_fails() {
        let text = COMPLETE.replace("- one reviewer\n", "\n");
        assert_eq!(missing_sections(&text), ["Review"]);
    }

    #[test]
    fn repeated_empty_heading_fails() {
        let text = format!("{COMPLETE}## Review\n\n");
        assert_eq!(missing_sections(&text), ["Review"]);
    }

    #[test]
    fn sample_agreement_is_complete() {
        assert!(missing_sections(include_str!("../samples/CLAUDE.md")).is_empty());
    }
}
