use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("You are playing 'Guess the number!'");
    
    println!("The secret number lies in range between x and y");

    println!("Enter x:");
    
    let mut min = String::new();

    loop {
        io::stdin().read_line(&mut min).expect("Failed to read line");
        let  min: u32 = match min.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number!");
                continue;
            },
        };
        break;
    };


    println!("Enter y:");

    let mut max = String::new();

    loop {
        io::stdin().read_line(&mut max).expect("Failed to read line");
        let max: u32 = match max.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number!");
                continue;
            },
        };
        break;
    };

    let x: u32 = min.trim().parse().expect("");
    let y: u32 = max.trim().parse().expect("");

    let secret_number = rand::thread_rng().gen_range(x..=y);

    //println!("The secret number is {secret_number}");
    
    println!("Enter number of attempts:");
    let mut attempts = String::new();
    io::stdin().read_line(&mut attempts).expect("Failed to read line");
    let attempts: u32 = attempts.trim().parse().expect("");

    for i in 1..=attempts {
        println!("Please input your guess. Attempt: {i}");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                return;
            }
        }
    };
    println!("You've ran out of attempts. You lose!");
}
