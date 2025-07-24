use crate::base::*;
use crate::parse_word_list::parse_word_list;
use rand::Rng;
use std::env;
use std::io::{self, Write};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reveal_word_basic() {
        let secret = "rustacean";
        let guessed = "rta";

        let expected: String = secret
            .chars()
            .map(|c| if guessed.to_lowercase().contains(c.to_ascii_lowercase()) { c } else { HIDDEN_CHARAKTER })
            .collect();

        let result = reveal_word(secret, guessed);
        assert_eq!(result, expected);
    }

    #[test]
    fn reveal_word_case_insensitive() {
        let secret = "Rustacean";
        let guessed = "RTA";

        let expected: String = secret
            .chars()
            .map(|c| if guessed.to_lowercase().contains(c.to_ascii_lowercase()) { c } else { HIDDEN_CHARAKTER })
            .collect();

        let result = reveal_word(secret, guessed);
        assert_eq!(result, expected);
    }

    #[test]
    fn reveal_word_no_guesses() {
        let secret = "hello";
        let guessed = "";

        // All characters replaced with HIDDEN_CHARAKTER:
        let expected = std::iter::repeat(HIDDEN_CHARAKTER).take(secret.len()).collect::<String>();

        let result = reveal_word(secret, guessed);
        assert_eq!(result, expected);
    }

    #[test]
    fn reveal_word_all_guessed() {
        let secret = "test";
        let guessed = "test";

        // All revealed, should be same as secret
        let expected = secret.to_string();

        let result = reveal_word(secret, guessed);
        assert_eq!(result, expected);
    }

    #[test]
    fn reveal_word_empty_secret() {
        let secret = "";
        let guessed = "abc";

        // Empty secret, expect empty string
        let expected = "".to_string();

        let result = reveal_word(secret, guessed);
        assert_eq!(result, expected);
    }

    #[test]
    fn reveal_word_wrong_test() {
        let secret = "ABCDEFG";
        let guessed = "abc";

        // Empty secret, expect empty string
        let expected = "".to_string();

        let result = reveal_word(secret, guessed);
        assert_eq!(result, expected);
    }
}

pub fn reveal_word(secret: &str, guessed_letters: &str) -> String {
    let guessed_lowercase = guessed_letters.to_lowercase();

    secret
        .chars()
        .map(|c| {
            if guessed_lowercase.contains(c.to_ascii_lowercase()) {
                c
            } else {
                HIDDEN_CHARAKTER
            }
        })
        .collect()
}

pub fn hang_man() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the Great Game of Hangman, were only one will have the great honor, to be hanged!");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Well, I guess you are a donkey, you didn't provide the secret secret file, for the challange! @#$%^&");
        return Err("Provide a file, you idiot!".into());
    }

    let words = parse_word_list(&args[1], HIDDEN_CHARAKTER)?;

    let word_i = rand::thread_rng().gen_range(0..words.len());
    let word = match words.get(word_i) {
        Some(w) => w,
        None => return Err(format!("Index out of Bounds \"{}\"", word_i).into()),
    };

    let mut guessed_letters = String::new();
    let mut hp = 5;

    loop {
        let guessed_word = reveal_word(word, &guessed_letters);

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

        if let Some(letter) = user_input.chars().next() {
            let letter_lower = letter.to_ascii_lowercase();

            if guessed_letters.to_ascii_lowercase().contains(letter_lower) {
                println!("You already guessed '{}'", letter);
                continue;
            }

            guessed_letters.push(letter);

            if word.to_ascii_lowercase().contains(letter_lower) {
                println!("Good guess!");
            } else {
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
