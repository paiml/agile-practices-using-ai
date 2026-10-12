//! Checks the frontmatter of a team skill (`skills/<name>/SKILL.md`).
//!
//! A skill is a packaged procedure the assistant loads by name. Without a
//! name and a description, the assistant cannot tell when to use it.
//! Contract: `contracts/team-skill-v1.yaml`.

/// The frontmatter keys every skill must set to a non-blank value.
pub const REQUIRED: [&str; 2] = ["name", "description"];

/// Contract equation `skill_frontmatter_complete`: returns the required keys
/// that are missing or blank. A file without a `---` frontmatter block
/// misses all of them.
pub fn missing_keys(text: &str) -> Vec<&'static str> {
    let front = frontmatter(text).unwrap_or_default();
    REQUIRED
        .iter()
        .copied()
        .filter(|key| {
            !front
                .iter()
                .any(|(k, v)| k == key && !v.trim_matches(['"', '\'', ' ']).is_empty())
        })
        .collect()
}

/// The `key: value` pairs between the opening and closing `---` lines.
fn frontmatter(text: &str) -> Option<Vec<(String, String)>> {
    let mut lines = text.lines();
    if lines.next()?.trim_end() != "---" {
        return None;
    }
    let mut out: Vec<(String, String)> = Vec::new();
    for line in lines {
        if line.trim_end() == "---" {
            return Some(out);
        }
        if line.starts_with(char::is_whitespace) {
            // A continuation line of a block scalar (`key: >` or `key: |`).
            if let Some((_, v)) = out.last_mut() {
                v.push(' ');
                v.push_str(line.trim());
            }
        } else if let Some((k, v)) = line.split_once(':') {
            let v = v.trim();
            let v = if [">", "|", ">-", "|-"].contains(&v) {
                ""
            } else {
                v
            };
            out.push((k.trim().to_string(), v.to_string()));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_frontmatter_passes() {
        assert!(
            missing_keys("---\nname: retro\ndescription: Run a retro.\n---\nbody\n").is_empty()
        );
    }

    /// FALSIFY-SKILL-001: a skill without a description fails.
    #[test]
    fn falsify_missing_description_fails() {
        assert_eq!(missing_keys("---\nname: retro\n---\n"), ["description"]);
    }

    /// FALSIFY-SKILL-002: an unclosed frontmatter block counts as none.
    #[test]
    fn falsify_unclosed_frontmatter_fails() {
        assert_eq!(missing_keys("---\nname: retro\ndescription: x\n"), REQUIRED);
    }

    #[test]
    fn block_scalar_needs_a_body() {
        assert_eq!(
            missing_keys("---\nname: retro\ndescription: >\n---\n"),
            ["description"]
        );
        assert!(missing_keys("---\nname: retro\ndescription: >\n  Run a retro.\n---\n").is_empty());
    }

    #[test]
    fn sample_skill_is_complete() {
        assert!(missing_keys(include_str!("../samples/skills/sprint-review/SKILL.md")).is_empty());
    }
}
