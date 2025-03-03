use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the game, Guess the Number!");
    let secret_number = rand::rng().random_range(1..=100);

    println!("Please input your guess. (1-100)");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: i32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You Win!"),
    }
}
