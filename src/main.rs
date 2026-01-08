use std::io;
use std::cmp::Ordering;
use rand::Rng;
use fancy_print::{FancyPrinter, Animation};
use std::time::Duration;

fn wizard_speak(text: &str) {
    let printer = FancyPrinter::builder()
        .animation(Animation::CharacterCycling)
        .time_delay(Duration::from_millis(1))
        .multi_line(false)
        .build();

    printer.print(text);
    println!();
}

fn main() {
    wizard_speak("Welcome to Guess the Number!");
    wizard_speak("I am Marvolo the wizard!");
    wizard_speak("You can't see me, but I can see you...");

    println!("Press Enter to continue...");
    io::stdin().read_line(&mut String::new()).expect("Failed to read input");

    wizard_speak("Nobody ever succeeded in defeating me!");
    wizard_speak("Select your difficulty and let chaos begin!");

    let difficulty = select_difficulty();
    let config = create_game_config(difficulty);

    wizard_speak(&format!("I'm thinking of a number between 1 and {}", config.max_number));

    let secret_number = rand::thread_rng().gen_range(1..=config.max_number);
    let mut attempts = 0;

    loop {
        println!("Attempts left: {}", config.max_attempts - attempts);

        let guess = read_guess();
        attempts += 1;

        if attempts >= config.max_attempts {
            wizard_speak("Game over! You lost!");
            wizard_speak(&format!("The number was {}", secret_number));
            wizard_speak("Ahahahaha! You're a loser, cannot even guess my secret number with the hints I give. I WIN!!!");
            break;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                wizard_speak("You're low, try a higher number!");
                print_hint(guess, secret_number, config.max_number);
            },
            Ordering::Greater => {
                wizard_speak("You're high, try a lower number!");
                print_hint(guess, secret_number, config.max_number);
            },
            Ordering::Equal => {
                wizard_speak("Wow! You really guessed my secret number. Are you a secret wizard or something?");
                print_achievement(attempts);
                break;
            }
        }
    }
}

fn print_hint(guess: u32, secret: u32, max_number: u32) {
    let difference = (guess as i32 - secret as i32).abs();

    let closeness = (difference as f32 / max_number as f32) * 100.0;

    if closeness <= 3.0 {
        wizard_speak("You're very near to the end folk!")
    } else if closeness <= 15.0 {
        wizard_speak("You're closer, but not that much!")
    } else if closeness <= 25.0 {
        wizard_speak("Actually you're quite in the range of my number!")
    } else {
        wizard_speak("You, loser! FAR AWAY!!!")
    }
}

fn print_achievement(attempts: u32) {
    match attempts {
        1 => wizard_speak("Oh, no! You defeated my game. What an idiot! No, this...
                        Cannot happen..."),
        2..=3 => wizard_speak("Well done boy, you defeated me too soon. I knew this was gonna happen..."),
        4..=5 => wizard_speak("Actually, not bad. But you can ascend more!"),
        _ => wizard_speak("You barely made it, but a win is a win, I suppose..."),
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
