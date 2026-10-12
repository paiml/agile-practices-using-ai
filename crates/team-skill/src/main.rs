//! `team-skill <SKILL.md>...`: exit 0 when every file has a complete
//! frontmatter, 1 when one does not, 2 on a usage or read error.

use std::process::ExitCode;

fn main() -> ExitCode {
    let paths: Vec<String> = std::env::args().skip(1).collect();
    if paths.is_empty() {
        eprintln!("usage: team-skill <SKILL.md>...");
        return ExitCode::from(2);
    }
    let mut failed = false;
    for path in &paths {
        let text = match std::fs::read_to_string(path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{path}: {e}");
                return ExitCode::from(2);
            }
        };
        let missing = team_skill::missing_keys(&text);
        if missing.is_empty() {
            println!("{path}: ok");
        } else {
            failed = true;
            println!("{path}: missing frontmatter: {}", missing.join(", "));
        }
    }
    if failed {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
