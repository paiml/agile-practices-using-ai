//! `backlog-rank <backlog.csv>`: prints the backlog in ranked order.

use std::process::ExitCode;

fn main() -> ExitCode {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: backlog-rank <backlog.csv>");
        return ExitCode::from(2);
    };
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{path}: {e}");
            return ExitCode::from(2);
        }
    };
    let items = match backlog_rank::parse(&text) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("{path}: {e}");
            return ExitCode::from(2);
        }
    };
    for (n, item) in backlog_rank::rank(&items).iter().enumerate() {
        println!(
            "{:>2}. {} (value {}, effort {}, risk {})",
            n + 1,
            item.id,
            item.value,
            item.effort,
            item.risk
        );
    }
    ExitCode::SUCCESS
}
