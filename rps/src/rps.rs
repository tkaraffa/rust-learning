use rand::prelude::*;
use std::cmp::Ordering;
use std::error::Error;
use std::fmt;

#[derive(Debug, Eq)]
pub struct Player {
    pub name: String,
    pub choice: String,
    tie: bool,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.tie {
            write!(f, "It was a tie. Both players threw {}.", self.choice)
        } else {
            write!(f, "{} won. They threw {}.", self.name, self.choice)
        }
    }
}

impl Error for Player {}

impl Player {
    fn get_choices() -> Vec<String> {
        vec![
            String::from("rock"),
            String::from("paper"),
            String::from("scissors"),
        ]
    }

    pub fn bad_choice(&self) -> String {
        format!(
            "Bad Choice: '{}'. Please choose a valid option: '{}'.",
            self.choice,
            Self::get_choices().join("', '")
        )
    }

    pub fn from_choice(name: String, choice: String) -> Result<Self, Self> {
        let tie: bool = false;
        if Self::get_choices().contains(&choice) {
            Ok(Self { name, choice, tie })
        } else {
            Err(Self { name, choice, tie })
        }
    }

    pub fn from_random(name: String) -> Result<Self, Self> {
        let mut rng = rand::thread_rng();
        let random_choice =
            Self::get_choices().choose(&mut rng).unwrap().to_string();
        Self::from_choice(name, random_choice)
    }

    pub fn play<'a>(&'a self, other: &'a Self) -> Self {
        let tie: &Self = &Self {
            name: String::from("Tie"),
            choice: String::from(&self.choice),
            tie: true,
        };
        let winner: &Self = {
            if self > other {
                self
            } else if self < other {
                other
            } else {
                tie
            }
        };
        Self {
            name: winner.name.to_string(),
            choice: winner.choice.to_string(),
            tie: winner.tie,
        }
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        self.choice.cmp(&other.choice)
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Self) -> bool {
        ((self.choice == "scissors") & (other.choice == "rock"))
            | ((self.choice == "rock") & (other.choice == "paper"))
            | ((self.choice == "paper") & (other.choice == "scissors"))
    }

    fn gt(&self, other: &Self) -> bool {
        ((self.choice == "paper") & (other.choice == "rock"))
            | ((self.choice == "rock") & (other.choice == "scissors"))
            | ((self.choice == "scissors") & (other.choice == "paper"))
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.choice == other.choice
    }
}

// tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_choice() {
        let good_choices: Vec<&str> = vec!["rock", "paper", "scisscors"];
        let choices: Vec<&str> =
            vec!["bad", "rock", "other bad", "paper", "SCISSORS"];
        for choice in choices {
            let test_player: Result<Player, Player> = Player::from_choice(
                String::from("Tester"),
                String::from(choice),
            );
            if good_choices.contains(&choice) {
                assert_eq!(
                    test_player.unwrap().choice,
                    String::from(choice).to_lowercase()
                )
            } else {
                assert!(test_player.is_err())
            };
        }
    }

    #[test]
    fn test_comparisons() {
        let player1_choices: Vec<&str> = vec!["rock", "paper", "scissors"];
        let player2: Result<Player, Player> =
            Player::from_choice(String::from("Tester"), String::from("rock"));
        for choice in player1_choices {
            let player1: Result<Player, Player> = Player::from_choice(
                String::from("Tester"),
                String::from(choice),
            );
            let conditions = {
                if choice == "rock" {
                    (
                        vec![player1 == player2],
                        vec![player1 > player2, player1 < player2],
                    )
                } else if choice == "paper" {
                    (
                        vec![player1 < player2],
                        vec![player1 == player2, player1 > player2],
                    )
                } else if choice == "scissors" {
                    (
                        vec![player1 > player2],
                        vec![player1 == player2, player1 < player2],
                    )
                } else {
                    (vec![], vec![])
                }
            };
            let (trues, falses) = conditions;

            for condition in trues {
                assert!(condition);
            }
            for condition in falses {
                assert!(!condition);
            }
        }
    }
}
