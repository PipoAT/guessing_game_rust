// add ability to accept user input
use std::io; 

fn main() {
    // println prints to the terminal/screen
    println!("Guess the number!");

    println!("Please input your guess.");

    // store user input into a string
    // mut = mutable
    // variables in rust are immutable by default
    // making them unchangable once a value is assigned
    // thus add mut will allow changes
    let mut guess = String::new();

    // handle the user input using the io library
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // print the user input
    println!("You guessed: {}", guess);
}
