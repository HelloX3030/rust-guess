use crate::base::*;

use rand::Rng;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::{self, Write};

pub fn hang_man() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the Great Game of Hangman, were only one will have the great honor, to be hanged!");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Well, I guess you are a donkey, you didn't provide the secret secret file, for the challange! @#$%^&");
        return Err("Provide a file, you idiot!".into());
    }

    let file = match File::open(&args[1]) {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to open file '{}': {}", &args[1], e);
            return Err(e.into());
        }
    };

    let mut words: Vec<String> = Vec::new();
    let reader = BufReader::new(file);
    for line_result in reader.lines() {
        let line = line_result?;
        let line = line.trim().to_string();
        if line.contains(HIDDEN_CHARAKTER) {
            return Err(format!("Found Limiter \"{}\"", HIDDEN_CHARAKTER).into());
        }
        if !line.is_empty() {
            words.push(line);
        }
    }
    if words.is_empty() {
        return Err("No words found!".into());
    }
    let word_i = rand::thread_rng().gen_range(0..words.len());
    let word = match words.get(word_i) {
        Some(w) => w,
        None => {
            return Err(format!("Index out of Bounds \"{}\"", word_i).into());
        }
    };
    let mut guessed_word = String::new();
    let mut hp = 5;
    for _ in word.chars() {
        guessed_word.push(HIDDEN_CHARAKTER);
    }
    loop {
        if guessed_word == *word {
            break;
        }
        println!("Current Word: {}", guessed_word);
        print!("HP: ");
        for _ in 0..hp {
            print!("♥");
        }
        println!();
        let mut user_input = String::new();
        print!("{}", PROMPT);
        io::stdout().flush()?;
        io::stdin().read_line(&mut user_input)?;
        let user_input = user_input.trim();
        if user_input == "exit" {
            return Ok(());
        } else if user_input.len() != 1 {
            println!("Invalid input!");
            continue;
        }
        if let Some(user_input) = user_input.chars().next() {
            let mut found: bool = false;
            let mut guessed_word_vec = guessed_word.chars().collect::<Vec<char>>();
            for (i, c) in word.chars().enumerate() {
                if c == user_input {
                    found = true;
                    guessed_word_vec[i] = user_input;
                }
            }
            guessed_word = guessed_word_vec.iter().collect();
            if !found {
                println!("Muhahahahahahahahahahah!");
                hp -= 1;
                if hp <= 0 {
                    return Err("Skill Issue!".into());
                }
            }
        } else {
            println!("No Valid User Input found");
            continue;
        }
    }
    println!("Ohhh noooooooo, you won this intense Bossfight!");
    Ok(())
}
