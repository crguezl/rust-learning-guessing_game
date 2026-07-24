use std::cmp::Ordering;
use std::io;

// use rand::Rng;
use rand::prelude::*;

fn compare_guess(guess: u32, secret_number: u32) -> &'static str {
    match guess.cmp(&secret_number) {
        Ordering::Less => "Too small!",
        Ordering::Greater => "Too big!",
        Ordering::Equal => "You win!",
    }
}

// A macro starts with the keyword `macro_rules!` followed by the name of the macro and a block
// containing the macro definition. The macro definition consists of one or more rules, 
// each of which specifies a pattern and a corresponding expansion. 
// The pattern is matched against the input provided to the macro, and if it matches, 
// the corresponding expansion is generated.
macro_rules! print_guess_info {
    ($guess:expr) => {{
        println!("  guess: stack (String): {:p}", &$guess);
        println!("  guess: heap  (buffer): {:p}", $guess.as_ptr());
        println!("  guess: len={}, cap={}", $guess.len(), $guess.capacity());
    }};
}

fn main() {
    println!(r"\tGuess the number\n!"); // raw string literal

    // 1..=100 is a range that includes both 1 and 100
    let secret_number = rand::rng().random_range(1..=100);
    // raw string literal with interpolation. No need to escape the double quotes!.
    println!(r#"  The secret number is: "{secret_number}""#);

    loop {
        let mut guess = String::new();

        print_guess_info!(guess);

        println!("Please, input your guess.");

        /* // Correct but a match statement is better
        let num_char = io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        */

        /*
        // Incorrect: num_char is a Result type, not an integer.
        let num_char = std::io::stdin()
            .read_line(&mut guess);
        */

        // match example
        let num_char = match io::stdin().read_line(&mut guess) {
            Ok(0 | 1) => {
                println!("No valid choice provided. Exiting...");
                return;
            }
            Ok(n) => n,
            Err(error) => {
                eprintln!("Error when reading line: {error}");
                return;
            }
        };
        // guess.trim return a &str, so we need to convert it to String
        guess = guess.trim().to_string();

        println!("  You guessed: {guess}");
        println!("  Number of characters: {:?}", num_char);

        print_guess_info!(guess);

        println!("  Comparing your guess {guess} with the secret number {secret_number} ...");
        // We can declare a variable with the same name as a previous variable, shadowing it.
        //This is useful when we want to transform a value but keep the same name.
        let guess = match guess.parse::<u32>() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                return;
            }
        };
        let result = compare_guess(guess, secret_number);
        println!("Result: {result}");
        if result == "You win!" {
            break;  
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(compare_guess(30, 30), "You win!");
        assert_eq!(compare_guess(20, 30), "Too small!");
        assert_eq!(compare_guess(40, 30), "Too big!");
    }
}
