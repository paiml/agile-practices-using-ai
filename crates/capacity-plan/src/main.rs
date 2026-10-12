//! `capacity-plan <estimates.csv> <capacity> <sprints>`: fills sprints in
//! backlog order and lists what does not fit.

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [path, capacity, sprints] = args.as_slice() else {
        eprintln!("usage: capacity-plan <estimates.csv> <capacity> <sprints>");
        return ExitCode::from(2);
    };
    let (Ok(capacity), Ok(sprints)) = (capacity.parse::<u32>(), sprints.parse::<usize>()) else {
        eprintln!("capacity and sprints must be whole numbers");
        return ExitCode::from(2);
    };
    let items = match std::fs::read_to_string(path)
        .map_err(|e| e.to_string())
        .and_then(|t| capacity_plan::parse(&t))
    {
        Ok(i) => i,
        Err(e) => {
            eprintln!("{path}: {e}");
            return ExitCode::from(2);
        }
    };
    let plan = capacity_plan::plan(&items, capacity, sprints);
    let load = capacity_plan::load(&items, &plan);
    for (n, ids) in plan.sprints.iter().enumerate() {
        println!(
            "sprint {}: {}/{capacity} points: {}",
            n + 1,
            load[n],
            ids.join(", ")
        );
    }
    println!(
        "overflow: {}",
        if plan.overflow.is_empty() {
            "none".to_string()
        } else {
            plan.overflow.join(", ")
        }
    );
    ExitCode::SUCCESS
}
