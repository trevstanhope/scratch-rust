extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Guess a number: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).ok().expect("Failed to readline");

        let guess: u32 = guess.trim().parse().ok().expect("Please type a number!");
        
        println!("Your guess: {}", guess);

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
