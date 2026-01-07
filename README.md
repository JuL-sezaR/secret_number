# Secret Number

This repository contains my **first-ever Rust project**
It is a simple **terminal-based guessing game** with a small twist.

You are facing a wizard who has chosen a **secret number**.  
You can choose between 4 different difficulty levels, 1 being the easiest and 4 being the hardest(simply impossible). Worth trying it out tho!
The wizard talks quite dark sometimes. I mean generally he does it a lot. Don't take him that serious.

---

## How the Game Works

- The wizard chooses a random number between **1 and x**
- You have **x attempts**
- After each guess, the game tells you:
  - Whether your guess is **too high** or **too low**
  - A **hint** about how close you are
- Guess the number in time to win and deffeat the wizard!🧙‍♂️

---

## Requirements

Before running the project, make sure you have:

- **Rust** installed (`rustc` and `cargo`)

To check:
```bash
rustc --version
cargo --version
```

Afterwards, make sure to clone the repository with "git clone https://github.com/JuL-sezaR/secret_number.git"
Then:
 - "cd secret_number"
 - "cargo run"
