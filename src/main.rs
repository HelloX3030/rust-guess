use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use rand::Rng;
use std::env;
use std::io::{self, Write};

const HIDDEN_CHARAKTER: char = '*';

fn main() ->Result<(), Box<dyn std::error::Error>> {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        print!("> ");
        io::stdout().flush()?;
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess = guess.trim();
        if guess == "exit" { break; }

        println!("You guessed: \"{}\"", guess);

        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("pls enter a number!");
                continue;
            }
        };

        if guess < secret_number {
            println!("Too small!");
        } else if guess > secret_number {
            println!("Too big!");
        } else {
            println!("It lookes like you are not a donkey, but the real challange begins now!");
            break;
        }
    }

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
            return  Err(format!("Found Limiter \"{}\"", HIDDEN_CHARAKTER).into());
        }
        if line != "" {
            words.push(line);
        }
    }
    if words.len() <= 0 {
        return  Err("No words found!".into());
    }
    let word_i = rand::thread_rng().gen_range(0..words.len());
    let word = match words.get(word_i) {
        Some(w) => w,
        None => {
            return Err(format!("Index out of Bounds \"{}\"", word_i).into());
        }
    };
    let mut guessed_word = String::new();
    for _ in word.chars() {
        guessed_word.push(HIDDEN_CHARAKTER);
    }
    println!("Current Word: {}", guessed_word);
    Ok(())
}
