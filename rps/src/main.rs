#![allow(dead_code, unused)] // debug
/// todo:
/// * add random selection for opponent choice
/// * add interactive mode if no args passed
/// * add structs for each player's choices
/// that have ways of comparing choices to determine winner
use clap::Parser;
// use rand::prelude;
// use rand::seq::IteratorRandom;
/// simple rock-paper-scissors game
#[derive(Parser, Debug)]
#[clap(author = "me")]
struct Args {
    /// player's choice
    choice: String,
}

fn main() {
    let args = Args::parse();

    let player_choice: &str = args.choice;
    let opponent_choice: &str = choose_opponent();

    let outcome: &str = rps((&player_choice, &opponent_choice))kw;
    println!(
        "Player picked {}, opponent picked {}",
        player_choice, opponent_choice
    );

    match &outcome {
        &"win" => println!("player won"),
        &"tie" => println!("it was a tie"),
        &"lose" => println!("player lost"),
        _ => println!("huh?"),
    }
}

fn choose_opponent() -> &'static str {
    // let choices = ["rock", "paper", "scissors"];
    // let mut rng = rand::thread_rng();
    // let opponent_choice = choices.choose(&mut rng).unwrap();
    let opponent_choice: &str = "scissors";
    opponent_choice
}

fn rps<'a>(choices: (&str, &str)) -> &'a str {
    match choices {
        ("rock", "rock") | ("paper", "paper") | ("scissors", "scissors") => &"tie",
        ("paper", "rock") | ("rock", "scissors") | ("scissors", "paper") => &"win",
        ("scissors", "rock") | ("rock", "paper") | ("paper", "scissors") => &"lose",
        other => &"huh?",
    }
}
