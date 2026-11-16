//! `story-contract <file>`: checks every story in the file. Exit 0 if all are
//! accepted, 1 if any is refused, 2 if the file cannot be read or parsed.

use std::process::ExitCode;

fn main() -> ExitCode {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: story-contract <stories.txt>");
        return ExitCode::from(2);
    };
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{path}: {e}");
            return ExitCode::from(2);
        }
    };
    let stories = match story_contract::parse(&text) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{path}: {e}");
            return ExitCode::from(2);
        }
    };
    if stories.is_empty() {
        eprintln!("{path}: no stories (an empty backlog is not accepted)");
        return ExitCode::from(2);
    }
    let mut refused = 0;
    for story in &stories {
        match story_contract::validate(story) {
            Ok(()) => println!("ok      {} ({} criteria)", story.id, story.criteria.len()),
            Err(e) => {
                println!("REFUSED {e}");
                refused += 1;
            }
        }
    }
    if refused == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
