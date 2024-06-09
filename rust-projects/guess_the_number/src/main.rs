use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the Number");

    loop {
        println!("Enter your guess.");

        let mut guess = String::new(); // Created mutable String type variable
        let cpu_guess = rand::thread_rng().gen_range(1..=100); //.gen_range belongs to Rng and 1-100 is inclusive

        io::stdin()
            .read_line(&mut guess) //reads input and stores in guess variable
            .expect("Cannot read number"); // When read_line returns Err type

        let guess: u32 = match guess.trim().parse() { // checking if user input is a number, here match tries to compare result of parse() which will either be Ok or Err
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number");
                continue;
            },
        };

        println!("Guessed Number : {guess}");

        match guess.cmp(&cpu_guess) { // match compares 2 expressions
            Ordering::Less => println!("Number less than CPU Guess {cpu_guess}"), //Less, Greater and Equal are patterns to which the cmp tries to match its result
            Ordering::Greater => println!("Number greater than CPU Guess {cpu_guess}"),
            Ordering::Equal => {
                println!("You guessed it right !! {cpu_guess}");
                break;
            },
        }
    }
}
