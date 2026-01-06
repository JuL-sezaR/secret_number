use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to Guess the Number!");
    println!("I'm thinking of a number between 1 and 1000");

    let secret_number = rand::thread_rng().gen_range(1..=1000);
    let mut attempts = 0;
    const MAX_ATTEMPTS: u32 = 7;

    loop {
        if attempts >= MAX_ATTEMPTS {
            println!("Game over! You lost!");
            println!("The number was {}", secret_number);
            break;
        }

        println!("Attempts left: {}", MAX_ATTEMPTS - attempts);
        println!("Enter your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("I can't read your input, you little mouse!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Are you drunk? Enter a valid number!");
                continue;
            }
        };

        attempts += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("You're low, try a higher number!");
                print_hint(guess, secret_number);
            },
            Ordering::Greater => {
                println!("You're high, try a lower number!");
                print_hint(guess, secret_number);
            },
            Ordering::Equal => {
                println!("Wow! You really guessed my secret number.
                    Are you a wizard or something?");
                print_achievement(attempts);
                break;
            }
        }
    }
}

fn print_hint(guess: u32, secret: u32) {
    let difference = (guess as i32 - secret as i32).abs();
    match difference {
        1..=5 => println!("You're very near to the end folk!"),
        6..=25 => println!("You're closer, but not that much!"),
        26..=50 => println!("Actually you're quite in the range of my number!"),
        _ => println!("You, loser! FAR AWAY!!!"),
    }
}

fn print_achievement(attempts: u32) {
    match attempts {
        1 => println!("Oh, no! You defeated my game. What an idiot! No, this...
                        Cannot happen..."),
        2..=3 => println!("Well done boy, you defeated me too soon. I knew this was gonna happen..."),
        4..=5 => println!("Actually, not bad. But you can ascend more!"),
        _ => println!("Ahahahaha! You're a loser, cannot even guess my secret number with the hints I give. I WIN!!!")
    }
}
