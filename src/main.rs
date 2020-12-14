extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret number = {}", secret_number);
    loop {
        println!("Please input your number:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("your guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    
    }
}
