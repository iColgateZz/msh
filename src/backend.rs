use std::io;
use std::process::ExitCode;

pub struct Command {
    pub program: String,
    pub args: Vec<String>,
}

pub fn run(command: &self::Command) -> io::Result<ExitCode> {
    let status = std::process::Command::new(&command.program)
        .args(&command.args)
        .status()?;

    if status.success() {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::FAILURE)
    }
}
