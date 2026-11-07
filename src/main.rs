mod backend;
mod builtin;
mod expand;
mod frontend;
mod glob;
mod shell;

use std::io::{self, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    let stdin = io::stdin();
    let mut shell = shell::Shell::new();
    let mut line = String::new();

    loop {
        print!("msh> ");
        if let Err(error) = io::stdout().flush() {
            eprintln!("msh: {error}");
            return ExitCode::FAILURE;
        }

        line.clear();
        match stdin.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => (),
            Err(error) => {
                eprintln!("msh: input: {error}");
                return ExitCode::FAILURE;
            }
        }

        if let Some(command) = frontend::parse(&line)
            && let shell::Outcome::Exit(_) = shell.evaluate(command)
        {
            break;
        }
    }

    shell.last_status
}
