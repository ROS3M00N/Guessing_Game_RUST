extern  crate rand;

use  std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..101);
    let mut count = 0;

    loop {
        println!("Please input yout guess: ");
    
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).
        expect("Failed to read line.");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
            //.expect("Please type a number: ");
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("YOU WIN!");
                break;
            }
        }
        count += 1;
    }
    println!("You take {} guesses.", count);
}