use std::process::ExitCode;

use crate::{backend, builtin, expand, frontend::ParsedCommand};

pub struct Shell {
    pub last_status: ExitCode,
}

pub enum Outcome {
    Continue(ExitCode),
    Exit(ExitCode),
}

impl Shell {
    pub fn new() -> Self {
        Self {
            last_status: ExitCode::SUCCESS,
        }
    }

    pub fn evaluate(&mut self, command: ParsedCommand) -> Outcome {
        let expanded = std::env::current_dir().and_then(|current_dir| {
            expand::expand(
                &command,
                &expand::Context {
                    home: std::env::var("HOME").ok(),
                    current_dir,
                },
            )
        });

        let outcome = match expanded {
            Err(error) => {
                eprintln!("msh: expansion: {error}");
                Outcome::Continue(ExitCode::FAILURE)
            }

            Ok(command) => match builtin::lookup(&command.program) {
                Some(builtin) => builtin::run(builtin, &command.args, self.last_status),

                None => match backend::run(&command) {
                    Ok(status) => Outcome::Continue(status),

                    Err(error) => {
                        eprintln!("msh: {}: {error}", command.program);
                        Outcome::Continue(ExitCode::FAILURE)
                    }
                },
            },
        };

        self.last_status = match outcome {
            Outcome::Continue(status) | Outcome::Exit(status) => status,
        };

        outcome
    }
}
