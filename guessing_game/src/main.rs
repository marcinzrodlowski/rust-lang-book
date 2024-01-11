use std::io; // input/output library, comes from standard library (std), has many useful features like ability to accept user input
use std::cmp::Ordering; // another feature from standard library for comparing values
use rand::Rng; // rand library for getting random functionality

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1, 101); // thread-local random number generator in range from 1 to 101 (explicitly)

    loop { // creates infinite loop
        println!("Please input your guess.");

        let mut guess = String::new(); // place to store the user input, returns a new instance of a string, :: means that new is associated function of the String type (not instance of a String, so in this case NOT on guess variable). This approach is called static method. The 'new' function creates a new, empty string. mut means that variable is mutable.

        io::stdin() // we are calling stdin function from the io module
            .read_line(&mut guess) // it puts whatever was typed in the terminal to the mutable variable provided inside the parenthesis. & means that this is a reference (allows multiple parts of the program to access data without needing to copy that data into memory multiple times). References are immutable by default thus we have to use &mut to make it mutable.
            .expect("Failed to read line"); // if the read_line method returns an Error, expect will cause the program to crash and display the message inside it.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number.");
                continue;
            }
        }; // shadowing allows us to overwrite/shadow value declared previously. trim(), will eliminate any whitespace in a string (also \n after pressing enter). parse(), parses a string into some kind of number but we need to tell rust the exact number type we want by using u32. Parse method returns Result type and in this case we are using match to handle this. If parse was able to successfull turn the string into number, it will return Ok that contains the number in other case it will return an error and keep continue asking for a number.

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // match expression is made up of 'arms'
            Ordering::Greater => println!("Too big!"), // arm
            Ordering::Equal => {
                println!("You win!");
                break; // will break from inifinite loop
            } // arm
        }
    }
}


// cargo update - ignores Cargo.lock and updates our dependencies
// cargo doc --open - if we want to see documentation for dependencies that we are using (offline)