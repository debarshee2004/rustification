use colored::*;
use rand::Rng;
// Import the Rng trait to generate random numbers
use std::cmp::Ordering;
// Import the Ordering enum to compare values
use std::io;
// Import the io module to handle user input/output, Import the colored crate to add color to terminal output

fn main() {
    // Print a welcome message to the user
    println!("Guessing Number Game!");

    // Generate a random number between 1 and 100 (inclusive) and store it in `secret_number`
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Start an infinite loop that will continue until the user guesses the correct number
    loop {
        // Prompt the user to input their guess
        println!("Please input your guess.");

        // Create a mutable variable `guess` to store the user's input
        let mut guess = String::new();

        // Read the user's input from the standard input (keyboard) and store it in `guess`
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // Handle any potential errors that occur during input

        // Attempt to parse the user's input into an unsigned 32-bit integer (u32)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // If parsing succeeds, store the number in `guess`
            Err(_) => {
                // If parsing fails, prompt the user to enter a valid number and continue the loop
                println!("Please enter a valid number!");
                continue;
            }
        };

        // Compare the user's guess to the secret number using the cmp method
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                // If the guess is equal to the secret number, print a "You win!" message in green and break out of the loop
                println!("{}", "You win!".green());
                break;
            }
            Ordering::Less => println!("{}", "Wrong Guess!".red()),
            // If the guess is less than the secret number, print "Wrong Guess!" in red
            Ordering::Greater => println!("{}", "Wrong Guess!".red()),
            // If the guess is greater than the secret number, print "Wrong Guess!" in red
        }

        // Print the user's guess and the secret number (for debugging purposes)
        println!("You guessed: {}", guess);
        println!("The secret number is: {}", secret_number);
    }
}
