use crate::base::*;
use rand::Rng;
use std::io::{self, Write};

pub fn guess_number() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the Great Game of Number Guessing! (Aka who is the donkey? You!)");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        print!("{}", PROMPT);
        io::stdout().flush()?;
        io::stdin().read_line(&mut guess)?;
        let guess = guess.trim();
        if guess == "exit" {
            return Ok(());
        }

        println!("You guessed: \"{}\"", guess);

        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("pls enter a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => {
                println!("Too small!");
            }
            std::cmp::Ordering::Greater => {
                println!("Too big!");
            }
            std::cmp::Ordering::Equal => {
                println!("It looks like you are not a donkey, but the real challenge begins now!");
                break;
            }
        }        
    }
    Ok(())
}
