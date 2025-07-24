use crate::base::*;
use rand::Rng;
use std::io::{self, Write};

pub fn guess_number() -> Result<(), Box<dyn std::error::Error>>{
    println!("Welcome to the Great Game of Number Guessing! (Aka who is the donkey? You!)");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        print!("{}", PROMPT);
        io::stdout().flush()?;
        io::stdin().read_line(&mut guess)?;
        let guess = guess.trim();
        if guess == "exit" { return Ok(()); }

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
    return Ok(())
}
