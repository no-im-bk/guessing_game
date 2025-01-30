use std::io;

fn main() {
    println!("Number guessing game");

    println!("Please guess an integer from 1 to 100");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed {}", guess);
}
