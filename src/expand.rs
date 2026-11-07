use std::io;
use std::path::PathBuf;

use crate::{backend, frontend::ParsedCommand, glob};

pub struct Context {
    pub home: Option<String>,
    pub current_dir: PathBuf,
}

pub fn expand(command: &ParsedCommand, context: &Context) -> io::Result<backend::Command> {
    let mut arguments = Vec::new();

    for word in &command.words {
        if word == "." {
            let directory = context.current_dir.to_str().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "current directory is not UTF-8")
            })?;

            arguments.push(directory.to_string());
            continue;
        }

        let value = if word == "~" || word.starts_with("~/") {
            let home = context.home.as_ref().ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, "HOME is not set or is not UTF-8")
            })?;

            format!("{home}{}", &word[1..])
        } else {
            word.clone()
        };

        arguments.extend(glob::expand(&value)?);
    }

    let mut arguments = arguments.into_iter();
    let program = arguments
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "command has no words"))?;

    Ok(backend::Command {
        program,
        args: arguments.collect(),
    })
}
