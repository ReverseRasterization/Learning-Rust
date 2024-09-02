//use std::io; // you may also use std::io::Stdin
use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //start..=end (inclusive)

    //println!("The secret number is {secret_number}");

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) //references (&) are immutable by default; hence, "&mut"             // Returns the Result enum; Ok and Err, Err contains information about how or why the operation failed.
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() { // trim eliminates white space and new lines ("\n")      // parse converts a string to another type
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! (average)"),
            Ordering::Greater => println!("Too big! (fucking gigantic)"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
//howdy