mod rps;
/// todo:
/// * add interactive mode if no args passed
/// * write tests
use clap::Parser;
use rps::Player;
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

    let player: Player = Player::from_choice(String::from("Player"), choice)
        .map_err(|e| e.bad_choice())?;
    let opponent: Player = Player::from_random(String::from("Computer"))
        .map_err(|e| e.bad_choice())?;

    let winner: Player = player.play(&opponent);
    println!("{}", winner);

    Ok(())

    // if not, go into interactive mode
    // get user input to play, exit, or continue loop
    // run rps against newest choice
    // keep score
}
