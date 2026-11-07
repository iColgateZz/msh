pub struct ParsedCommand {
    pub words: Vec<String>,
}

pub fn parse(line: &str) -> Option<ParsedCommand> {
    let words: Vec<String> = line.split_whitespace().map(String::from).collect();

    if words.is_empty() {
        None
    } else {
        Some(ParsedCommand { words })
    }
}
