use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to the game, Guess the Number!");
    let secret_number = rand::rng().random_range(1..=100);

    println!("Please input your guess. (1-100)");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("The secret number is: {secret_number}");
    println!("You guessed: {}", guess);
}
