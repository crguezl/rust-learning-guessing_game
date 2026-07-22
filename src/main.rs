//use std::io;

fn main() {
    println!("Guess the number!");

    let mut guess = String::new();

    println!("stack (String): {:p}", &guess);
    println!("heap  (buffer): {:p}", guess.as_ptr());
    println!("len={}, cap={}", guess.len(), guess.capacity());

    println!("Please, input your guess.");

    /* // Correct but a mtach statement is better
    let num_char = std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
    */

    /* 
    // Incorrect: num_char is a Result type, not an integer. 
    let num_char = std::io::stdin()
        .read_line(&mut guess);
    */

    // match example
    let num_char = match std::io::stdin().read_line(&mut guess) {
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
}
