// add ability to accept user input
use std::io; 
// add ability to generate random numbers
use rand::Rng;
// add ability to compare using less, greater, or equal
use std::cmp::Ordering;

fn main() {
    // println prints to the terminal/screen
    println!("Guess the number!");

    // generate a number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop { // the loop keyword allows infinite looping

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

        // add handling in case input is not a numeric value
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // print the user input
        println!("You guessed: {}", guess);

        // compare guess to secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
