//! `five-whys <retro.txt>`: exit 0 when the chain is well formed, 1 when it
//! is refused, 2 on a usage or read error.

use std::process::ExitCode;

fn main() -> ExitCode {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: five-whys <retro.txt>");
        return ExitCode::from(2);
    };
    let chain = match std::fs::read_to_string(&path)
        .map_err(|e| e.to_string())
        .and_then(|t| five_whys::parse(&t))
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{path}: {e}");
            return ExitCode::from(2);
        }
    };
    match five_whys::validate(&chain) {
        Ok(()) => {
            println!(
                "{path}: ok, root cause: {}",
                chain.whys.last().map_or("", String::as_str)
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            println!("{path}: REFUSED: {e}");
            ExitCode::from(1)
        }
    }
}
