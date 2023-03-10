use rand::prelude::*;
use std::cmp::Ordering;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug, Eq)]
pub struct Player {
    pub choice: String,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self)
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
    pub fn get_choice(self) -> String {
        self.choice
    }

    pub fn from_choice(choice: String) -> Result<Player, Player> {
        if Player::get_choices().contains(&choice) {
            Ok(Player { choice: choice })
        } else {
            Err(Player { choice: choice })
        }
    }

    pub fn from_random() -> Result<Player, Player> {
        let mut rng = rand::thread_rng();
        let random_choice =
            Player::get_choices().choose(&mut rng).unwrap().to_string();
        Player::from_choice(random_choice)
    }

    // pub fn new(choice: String) -> Result<Player, Player> {
    //     Player::from_choice(choice)
    // }
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
