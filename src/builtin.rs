use std::process::ExitCode;

use crate::shell;

pub enum Builtin {
    Cd,
    Exit,
}

pub fn lookup(program: &str) -> Option<Builtin> {
    match program {
        "cd" => Some(Builtin::Cd),
        "exit" => Some(Builtin::Exit),
        _ => None,
    }
}

pub fn run(builtin: Builtin, args: &[String], last_status: ExitCode) -> shell::Outcome {
    match builtin {
        Builtin::Exit => shell::Outcome::Exit(last_status),

        Builtin::Cd => {
            if args.len() > 1 {
                eprintln!("msh: cd: too many arguments");
                return shell::Outcome::Continue(ExitCode::FAILURE);
            }

            let directory = args.first().cloned().or_else(|| std::env::var("HOME").ok());

            let Some(directory) = directory else {
                eprintln!("msh: cd: HOME is not set");
                return shell::Outcome::Continue(ExitCode::FAILURE);
            };

            match std::env::set_current_dir(&directory) {
                Ok(()) => shell::Outcome::Continue(ExitCode::SUCCESS),

                Err(error) => {
                    eprintln!("msh: cd: {}: {error}", directory);
                    shell::Outcome::Continue(ExitCode::FAILURE)
                }
            }
        }
    }
}
