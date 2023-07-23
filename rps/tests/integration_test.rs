use rps::Player;

#[test]
fn test_game() {
    let opponent =
        Player::from_choice(String::from("Computer"), String::from("rock"));
    let choices: Vec<&str> = vec!["rock", "paper", "scissors", "bad", ""];
    for choice in choices {
        let player =
            Player::from_choice(String::from("Player"), String::from(choice));
        match choice {
            "rock" | "paper" | "scissors" => {
                let winner: Player =
                    player.unwrap().play(opponent.as_ref().unwrap());
                match choice {
                    "rock" => assert!(winner.name == "Tie"),
                    "paper" => assert!(winner.name == "Player"),
                    "scissors" => assert!(winner.name == "Computer"),
                    _ => panic!(),
                }
            }
            _ => assert!(player.is_err()),
        };
    }
}
