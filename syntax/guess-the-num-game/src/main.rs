use std::io; //Use standard library for input and output
use rand::Rng; //Random number generation

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new(); //create empty string

    io::stdin() //get standard input hundle
        .read_line(&mut guess) //subject omission (Method Chain). "&" denotes a reference
        .expect("Failed to read line");

    match secret_number.cmp(&50) {//.cmp() returns "std::cmp::Ordering::Less, Greater, or Equal".
        std::cmp::Ordering::Less => println!("Too small!"),
        std::cmp::Ordering::Greater => println!("Too big!"),
        std::cmp::Ordering::Equal => println!("You win!"),
    }

    println!("You guessed: {}", guess);
}