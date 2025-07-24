use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_word_list(path: &str, forbidden_char: char) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut words = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let line = line.trim().to_string();
        if line.contains(forbidden_char) {
            return Err(format!("Found Limiter \"{}\"", forbidden_char).into());
        }
        if !line.is_empty() {
            words.push(line);
        }
    }

    if words.is_empty() {
        return Err("No words found!".into());
    }

    Ok(words)
}
