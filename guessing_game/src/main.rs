extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_num = rand::thread_rng().gen_range(1, 101);

    println!("Secret number is {}", secret_num);

    let mut ans = 0;

    loop {
        println!("Please input your guess #{}.", ans);

        // mut keyword let this variable "mutable".
        // By default all variable are being imuutable.
        let mut guess = String::new();

        // & indicates that this guess argument is a "Reference"
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Validate user input then cast a type.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { println!("Invalid input!"); continue }
        }; // Don't forget a semicolon here, Otherwise, compile errors raised.

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        ans += 1;

        match ans {
            4 => { println!("You lose!"); break },
            _ => continue,
        }
    }
}
