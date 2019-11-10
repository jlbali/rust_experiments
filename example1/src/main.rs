extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Input your guess:");
    let mut attempts = 0;
    let secret_number = rand::thread_rng().gen_range(1,101);
    loop{
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).
            expect("Failed to read line");
        attempts = attempts + 1;
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct after {} attempts!", attempts);
                break;
            },
        }
    }
}
