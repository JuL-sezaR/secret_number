use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to Guess the Number!");
    println!("I am Marvolo the wizard!");
    println!("You can't see me, but I can see you...");
    println!("Press Enter to continue...");
    io::stdin().read_line(&mut String::new()).expect("Failed to read input");
    println!("Nobody ever succeeded in defeating me!");
    println!("Select your difficulty and let chaos begin!");

    let difficulty = select_difficulty();
    let config = create_game_config(difficulty);

    println!("I'm thinking of a number between 1 and {}", config.max_number);

    let secret_number = rand::thread_rng().gen_range(1..=config.max_number);
    let mut attempts = 0;

    loop {
        if attempts >= config.max_attempts {
            println!("Game over! You lost!");
            println!("The number was {}", secret_number);
            break;
        }

        println!("Attempts left: {}", config.max_attempts - attempts);

        let guess = read_guess();
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
                    Are you a secret wizard or something?");
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

// New update, here I will be setting some new features, just like difficulty and secret items.
// Difficulty levels will be added, ranging from easy to hard.
// Secret items will be introduced, which can be found by the player and used to gain an advantage.

enum Difficulty {
    Easy,
    Medium,
    Hard,
    Unfeasible,
}

fn select_difficulty() -> Difficulty {
    loop {
        println!("Welcome again, warrior...");
        println!("Here, choose your difficulty:");
        println!("1. Easy, you're bad");
        println!("2. Decent, you do not trust yourself enough");
        println!("3. Hard, that's better");
        println!("4. UNFEASIBLE!!!");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("I cannot understand your choice juggler!");

        match choice.trim() {
            "1" => return Difficulty::Easy,
            "2" => return Difficulty::Medium,
            "3" => return Difficulty::Hard,
            "4" => return Difficulty::Unfeasible,
            _ => println!("Enter a number between 1 and 4 silly human!"),
        }
    }
}

struct GameConfig {
    max_number: u32,
    max_attempts: u32,
}

fn create_game_config(difficulty: Difficulty) -> GameConfig {
    match difficulty {
        Difficulty::Easy => GameConfig { max_number: 100, max_attempts: 10 },
        Difficulty::Medium => GameConfig { max_number: 500, max_attempts: 8 },
        Difficulty::Hard => GameConfig { max_number: 1000, max_attempts: 6 },
        Difficulty::Unfeasible => GameConfig { max_number: 10_000, max_attempts: 3 },
    }
}

fn read_guess() -> u32 {
    loop {
        print!("Enter your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("I don't understand, which part about this simple game you cannot understand, you garbage...");

        match guess.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("For god's sake, enter a valid number!"),
        }
    }
}
