use std::io;
use std::cmp::Ordering;

// use rand::Rng;
use rand::prelude::*;

fn compare_guess(guess: u32, secret_number: u32) -> &'static str {
    match guess.cmp(&secret_number) {
        Ordering::Less => "Too small!",
        Ordering::Greater => "Too big!",
        Ordering::Equal => "You win!",
    }
}

fn main() {
    println!(r"\tGuess the number\n!"); // raw string literal

    // let secret_number = thread_rng().gen_range(1..=100);
    let secret_number = rand::rng().random_range(1..=100);

    println!(r#"The secret number is: "{secret_number}""#); // raw string literal with interpolation. No need to escape the double quotes!.

    let mut guess = String::new();

    println!("stack (String): {:p}", &guess);
    println!("heap  (buffer): {:p}", guess.as_ptr());
    println!("len={}, cap={}", guess.len(), guess.capacity());

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

    println!("You guessed: {guess}");
    println!("Number of characters: {:?}",num_char);


    println!("stack (String): {:p}", &guess);
    println!("heap  (buffer): {:p}", guess.as_ptr());
    println!("len={}, cap={}", guess.len(), guess.capacity());

    println!("Comparing your guess with the secret number...");
    let guess_number = match guess.trim().parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };
    let result = compare_guess(guess_number, secret_number);
    println!("Result: {result}");
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
