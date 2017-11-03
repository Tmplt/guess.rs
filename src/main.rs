extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn get_input() -> Result<u32, String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<u32>() {
        Ok(integer) => Ok(integer),
        Err(_) => Err(String::from("input is not an integer!")),
    }
}

fn main() {
    println!("Guess the number! Please input an integer");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut tries = 0;

    loop {
        tries += 1;

        let guess = match get_input() {
            Ok(integer) => integer,
            Err(reason) => {
                println!("in parsing u32, {}", reason);
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You won after {} tries!", tries);
                break;
            }
        }

        println!("Current amount of tries: {}", tries);
    }
}
