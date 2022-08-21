use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("You are playing 'Guess the number!'\nThe secret number lies in range between min and max\nEnter min:");
    let min: u32 = get_num();
    println!("Enter max:");
    let max:u32 = get_num();

    let secret_num: u32 = rand::thread_rng().gen_range(min..=max);
    println!("Generated a secret a number in range between {min} and {max}");

    println!("Enter number of attempts to guess the secret number:");
    let attempts: u32 = get_num();
    let mut guess: u32;

    for i in 1..=attempts {
        println!("Attempt: {i}\nEnter your guess:");
        guess = get_num();
        
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                return;
            }
        }
    }
    
    println!("You lose. The secret number was {secret_num}");
}

fn get_num() -> u32 {
    let mut temp = String::new();
    let num: u32;
    loop {
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        num = match temp.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Input a number");
                continue;
            },
        };
        break;
    }
    num
}
