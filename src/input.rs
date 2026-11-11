use std::env;
use std::io::{self, IsTerminal, Read};

/// Returns the task from the command-line arguments, or exits with usage info if missing.
pub fn task_from_args() -> String {
    let task = env::args().skip(1).collect::<Vec<_>>().join(" ");

    if task.trim().is_empty() {
        let program = env::args().next().unwrap_or_else(|| "agents".to_string());
        eprintln!("Usage: {program} <task>");
        eprintln!("   or: cat <files> | {program} <task>");
        std::process::exit(1);
    }

    task
}

/// Reads everything from stdin if it is piped, otherwise returns an empty string.
pub fn read_piped_input() -> io::Result<String> {
    let stdin = io::stdin();
    if stdin.is_terminal() {
        return Ok(String::new());
    }
    let mut buf = String::new();
    stdin.lock().read_to_string(&mut buf)?;
    Ok(buf)
}
