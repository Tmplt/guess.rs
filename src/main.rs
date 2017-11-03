extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::collections::HashMap;

fn main() {
    println!("Guess the number! Please input an integer");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut tries : HashMap<u32, String> = HashMap::new();
    let mut counter = 0;

    loop {
        counter += 1;
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        tries.insert(counter, guess.clone());

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => {
                println!("That wasn't an integer");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You won after {} tries! History:", counter);

                let mut try = counter;
                for _ in 0..3 {
                    if !tries.contains_key(&try) {
                        break;
                    }

                    let guess = tries.get(&try).unwrap();
                    let len = guess.len() - 1; // don't print the entered newline
                    println!("{}: {}", try, &guess[..len]);

                    try -= 1;
                }

                break;
            }
        }
    }
}
