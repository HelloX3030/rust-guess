// user modules
mod base;
mod guess_number;
mod hang_man;

use guess_number::guess_number;
use hang_man::hang_man;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = guess_number() {
        println!("Number Guessing Failed: {}", e);
    }
    if let Err(e) = hang_man() {
        println!("Hang Man Failed: {}", e);
    }
    Ok(())
}
