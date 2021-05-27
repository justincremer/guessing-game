use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    game()
}

fn game() {
    let (min, max): (u32, u32) = (1, 10);
    let answer: u32 = rand::thread_rng().gen_range(min..max + 1);
    let mut guess = String::new();

    println!("Pick a number between {} and {}", min, max);
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    let guess: u32 = guess.trim().parse().expect("Input must be a real number");
    match guess.cmp(&answer) {
        Ordering::Greater => lose(answer),
        Ordering::Less => lose(answer),
        Ordering::Equal => println!("You win!"),
    }
}

fn lose(answer: u32) {
    let mut res = String::new();

    println!("You lose, the number was {}", answer);
    println!("Would you like to play again? [y/n]");
    io::stdin()
        .read_line(&mut res)
        .expect("Failed to read input");

    if res.trim().to_lowercase() == "y" {
        if !(std::process::Command::new("clear")
            .status()
            .unwrap()
            .success())
        {
            println!("Failed to clear console");
            return;
        }
        game();
        return;
    }

    println!("Thanks for playing, better luck next time");
}
