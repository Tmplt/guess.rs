extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number! Please input an integer");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut tries : Vec<(u32, String)> = Vec::new();
    let mut counter = 0;

    loop {
        counter += 1;
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        tries.push((counter, guess.clone()));

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

                for _ in 0..2 { // print history twice
                    for &(try, ref guess) in tries.iter() {
                        let len = guess.len() - 1; // don't print newline
                        println!("{}: {}", try, &guess[..len]);
                    }
                }

                break;
            }
        }
    }
}
