#![allow(dead_code, unused)] // debug
mod rps;
use crate::rps::Player;
/// todo:
/// * add interactive mode if no args passed
use clap::Parser;
use rand::prelude::*;
use rand::seq::IteratorRandom;

/// simple rock-paper-scissors game
#[derive(Parser, Debug)]
#[clap(author = "me")]
struct Args {
    /// player's choice
    choice: String,
}

fn main() {
    let args = Args::parse();

    // if specified on command line
    let player = Player::new(args.choice.to_string().to_lowercase());
    let outcome: String = rps(player);
    println!("{}", outcome);

    // if not, go into interactive mode
    // get user input to play, exit, or continue loop
    // run rps against newest choice
    // keep score
}

fn rps(player: Player) -> String {
    let opponent = Player::new(choose_opponent());

    let outcome: &str = {
        if player > opponent {
            "player wins"
        } else if player < opponent {
            "opponent wins"
        } else if player == opponent {
            "tie"
        } else {
            "something went wrong"
        }
    };

    outcome.to_string()
}

fn choose_opponent() -> String {
    let choices = vec!["rock", "paper", "scissors"];
    let mut rng = rand::thread_rng();
    let opponent_choice = choices.choose(&mut rng).unwrap();
    opponent_choice.to_string()
}
