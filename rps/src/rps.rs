use rand::prelude::*;
use std::cmp::Ordering;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug, Eq)]
pub struct Player {
    pub name: String,
    pub choice: String,
    _private: (),
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if self.name == String::from("Tie") {
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
    pub fn bad_choice(self) -> String {
        format!(
            "Bad Choice: '{}'. Please choose a valid option: '{}'.",
            self.choice,
            Self::get_choices().join("', '")
        )
    }

    pub fn from_choice(
        name: String,
        choice: String,
    ) -> Result<Player, Player> {
        let _private = ();
        if Player::get_choices().contains(&choice) {
            Ok(Player {
                name,
                choice,
                _private,
            })
        } else {
            Err(Player {
                name,
                choice,
                _private,
            })
        }
    }

    pub fn from_random(name: String) -> Result<Player, Player> {
        let mut rng = rand::thread_rng();
        let random_choice =
            Player::get_choices().choose(&mut rng).unwrap().to_string();
        Player::from_choice(name, random_choice)
    }

    pub fn play<'a>(&'a self, other: &'a Self) -> Self {
        let tie: &Player = &Player {
            name: String::from("Tie"),
            choice: String::from(&self.choice),
            _private: (),
        };
        let winner: &Player = {
            if self > other {
                self
            } else if self < other {
                other
            } else {
                tie
            }
        };
        Player {
            name: winner.name.to_string(),
            choice: winner.choice.to_string(),
            _private: (),
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

    fn le(&self, other: &Self) -> bool {
        (self.choice < (other.choice)) | (self.choice == other.choice)
    }
    fn ge(&self, other: &Self) -> bool {
        (self.choice > (other.choice)) | (self.choice == other.choice)
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.choice == other.choice
    }
}
