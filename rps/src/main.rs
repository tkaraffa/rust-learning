#![allow(dead_code, unused)] // debug
mod rps;
use crate::rps::Player;
/// todo:
/// * add interactive mode if no args passed
use clap::Parser;

/// simple rock-paper-scissors game
#[derive(Parser, Debug)]
#[clap(author = "me")]
struct Args {
    /// Player's throw for this round; can be "rock", "paper", or "scissors".
    choice: String,
}

fn main() {
    let args = Args::parse();
    let choice: String = args.choice.to_string().to_lowercase();

    // if specified on command line
    let player: Player = Player::from_choice(choice);
    let outcome: String = rps(player);
    println!("{}", outcome);

    // if not, go into interactive mode
    // get user input to play, exit, or continue loop
    // run rps against newest choice
    // keep score
}

fn rps(player: Player) -> String {
    let opponent = Player::from_random();

    let outcome: String = {
        if player > opponent {
            "player wins"
        } else if player < opponent {
            "opponent wins"
        } else if player == opponent {
            "tie"
        } else {
            "something went wrong"
        }
    }
    .to_string();

    outcome
}
