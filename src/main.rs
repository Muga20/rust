use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The Secret number is: {}", secret_number);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please enter a valid number!".red());
                continue
            }
        };

        println!("Your guessed : {}" , guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too Big".red()),
            Ordering::Equal=> {
                println!("{}","You win".green());
                break;
            }
        }
    }

}
