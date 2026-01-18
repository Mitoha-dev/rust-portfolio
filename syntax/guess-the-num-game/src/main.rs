use std::io; //Use standard library for input and output

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new(); //create empty string

    io::stdin() //get standard input hundle
        .read_line(&mut guess) //subject omission (Method Chain). "&" denotes a reference
        .expect("Failed to read line");

        println!("You guessed: {}", guess);
}