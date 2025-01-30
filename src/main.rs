use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Number guessing game");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please guess an integer from 1 to 100");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {println!("WINNER!!"); break;}
        }
    }
}
