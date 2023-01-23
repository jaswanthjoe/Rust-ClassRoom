// Rust by default declares few set of standard libraries in the scope of every program i.e., Prelude and the rest we need to declare them explicitly
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() { // Main function body
    println!("Okay. Guess the number!");

    let hidden_number = rand::thread_rng().gen_range(1..=101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); //a mutable variable is declared and an empty string is assigned to it

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() { //error handing
            Ok(num) => num, 
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&hidden_number) { //secret number and the guess is matched using cmp
            Ordering::Less => println!("Too small!"), //if the guess is below the secret number "too small" is displayed 
            Ordering::Greater => println!("Too big!"), //if the guess is above the secret number "too small" is displayed
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}


