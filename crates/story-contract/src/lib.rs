//! A backlog item turned into a user story whose acceptance criteria are data.
//!
//! An AI assistant can draft stories from raw notes in seconds. This crate is
//! the part that does not trust the draft: a story is accepted only when every
//! acceptance criterion has a Given, a When and a Then, and there is at least
//! one of them. Contract: `contracts/story-contract-v1.yaml`.

use std::fmt;

/// One acceptance criterion in Given/When/Then form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Criterion {
    pub given: String,
    pub when: String,
    pub then: String,
}

/// A user story with its acceptance criteria.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Story {
    pub id: String,
    pub title: String,
    pub criteria: Vec<Criterion>,
}

/// Why a story was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoryError {
    /// The story has no acceptance criteria at all.
    NoCriteria { id: String },
    /// A criterion is missing one of its three parts (`part` is given, when or then).
    EmptyPart {
        id: String,
        index: usize,
        part: &'static str,
    },
    /// A line could not be read.
    Parse { line: usize, reason: String },
}

impl fmt::Display for StoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoCriteria { id } => write!(f, "{id}: no acceptance criteria"),
            Self::EmptyPart { id, index, part } => {
                write!(f, "{id}: criterion {} has an empty `{part}`", index + 1)
            }
            Self::Parse { line, reason } => write!(f, "line {line}: {reason}"),
        }
    }
}

impl std::error::Error for StoryError {}

/// Contract equation `story_has_ac`: at least one criterion, and every part of
/// every criterion is non-blank.
pub fn validate(story: &Story) -> Result<(), StoryError> {
    if story.criteria.is_empty() {
        return Err(StoryError::NoCriteria {
            id: story.id.clone(),
        });
    }
    for (index, c) in story.criteria.iter().enumerate() {
        for (part, text) in [("given", &c.given), ("when", &c.when), ("then", &c.then)] {
            if text.trim().is_empty() {
                return Err(StoryError::EmptyPart {
                    id: story.id.clone(),
                    index,
                    part,
                });
            }
        }
    }
    Ok(())
}

/// Parses the plain-text story format used in `samples/`:
///
/// ```text
/// STORY-1: As a team lead I want ... so that ...
///   Given ... | When ... | Then ...
/// ```
///
/// A story line starts at column 0 with `<id>: <title>`; each indented line
/// is one criterion. Blank lines and lines starting with `#` are skipped.
pub fn parse(text: &str) -> Result<Vec<Story>, StoryError> {
    let mut stories: Vec<Story> = Vec::new();
    for (n, raw) in text.lines().enumerate() {
        let line = n + 1;
        if raw.trim().is_empty() || raw.trim_start().starts_with('#') {
            continue;
        }
        if raw.starts_with(char::is_whitespace) {
            let story = stories.last_mut().ok_or(StoryError::Parse {
                line,
                reason: "criterion before any story".into(),
            })?;
            story.criteria.push(parse_criterion(raw.trim(), line)?);
        } else {
            let (id, title) = raw.split_once(':').ok_or(StoryError::Parse {
                line,
                reason: "expected `<id>: <title>`".into(),
            })?;
            stories.push(Story {
                id: id.trim().to_string(),
                title: title.trim().to_string(),
                criteria: Vec::new(),
            });
        }
    }
    Ok(stories)
}

fn parse_criterion(text: &str, line: usize) -> Result<Criterion, StoryError> {
    let parts: Vec<&str> = text.split('|').map(str::trim).collect();
    let [given, when, then] = parts.as_slice() else {
        return Err(StoryError::Parse {
            line,
            reason: "expected `Given .. | When .. | Then ..`".into(),
        });
    };
    let strip = |s: &str, kw: &str| -> Result<String, StoryError> {
        s.strip_prefix(kw)
            .filter(|r| r.is_empty() || r.starts_with(char::is_whitespace))
            .map(|r| r.trim().to_string())
            .ok_or(StoryError::Parse {
                line,
                reason: format!("expected `{kw}`"),
            })
    };
    Ok(Criterion {
        given: strip(given, "Given")?,
        when: strip(when, "When")?,
        then: strip(then, "Then")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn story(criteria: Vec<Criterion>) -> Story {
        Story {
            id: "S-1".into(),
            title: "t".into(),
            criteria,
        }
    }

    fn crit(g: &str, w: &str, t: &str) -> Criterion {
        Criterion {
            given: g.into(),
            when: w.into(),
            then: t.into(),
        }
    }

    #[test]
    fn complete_story_is_accepted() {
        assert_eq!(
            validate(&story(vec![crit(
                "a board",
                "I add a card",
                "it is in To Do"
            )])),
            Ok(())
        );
    }

    /// FALSIFY-STORY-001: a story with no criteria is refused.
    #[test]
    fn falsify_story_without_criteria_is_refused() {
        assert_eq!(
            validate(&story(vec![])),
            Err(StoryError::NoCriteria { id: "S-1".into() })
        );
    }

    /// FALSIFY-STORY-002: a criterion with a blank part is refused.
    #[test]
    fn falsify_blank_then_is_refused() {
        let err = validate(&story(vec![crit("a board", "I add a card", "  ")]));
        assert_eq!(
            err,
            Err(StoryError::EmptyPart {
                id: "S-1".into(),
                index: 0,
                part: "then"
            })
        );
    }

    #[test]
    fn parse_reads_sample() {
        let text = include_str!("../samples/stories.txt");
        let stories = parse(text).expect("sample parses");
        assert!(!stories.is_empty());
        assert!(stories.iter().all(|s| validate(s).is_ok()));
    }

    #[test]
    fn keyword_must_be_a_whole_word() {
        assert!(parse("S-1: t\n  Givenx a | When b | Then c").is_err());
    }

    #[test]
    fn parse_refuses_criterion_without_story() {
        assert!(matches!(
            parse("  Given a | When b | Then c"),
            Err(StoryError::Parse { line: 1, .. })
        ));
    }
}
