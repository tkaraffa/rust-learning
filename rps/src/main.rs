mod rps;
use crate::rps::Player;
/// todo:
/// * add interactive mode if no args passed
/// * write tests
use clap::Parser;
use std::error::Error;

/// simple rock-paper-scissors game
#[derive(Parser, Debug)]
#[clap(author = "Tom Karaffa")]
struct Args {
    /// Player's throw for this round; can be "rock", "paper", or "scissors".
    choice: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let choice: String = args.choice.to_string().to_lowercase();

    // if specified on command line

    let player: Player =
        Player::from_choice(choice).map_err(|e| e.bad_choice())?;
    let opponent: Player =
        Player::from_random().map_err(|e| e.bad_choice())?;

    let outcome: String = rps(&player, &opponent);
    println!("{outcome}");

    Ok(())

    // if not, go into interactive mode
    // get user input to play, exit, or continue loop
    // run rps against newest choice
    // keep score
}

fn rps(player1: &Player, player2: &Player) -> String {
    let throws: String = format!(
        "Player 1 threw {}. Player 2 threw {}. ",
        player1.choice, player2.choice
    );
    let outcome: String = {
        if player1 > player2 {
            "Player 1 wins."
        } else if player1 < player2 {
            "Player 2 wins."
        } else if player1 == player2 {
            "It was a tie."
        } else {
            "something went wrong"
        }
    }
    .to_string();

    throws + &outcome
}
