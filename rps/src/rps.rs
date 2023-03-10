use rand::prelude::*;
use rand::seq::IteratorRandom;
use std::cmp::Ordering;

#[derive(Eq)]
pub struct Player {
    choice: String,
}

impl Player {
    fn get_choices() -> Vec<String> {
        vec![
            String::from("rock"),
            String::from("paper"),
            String::from("scissors"),
        ]
    }

    fn validate_choice(choice: String) -> Result<String, String> {
        if Player::get_choices().contains(&choice) {
            Ok(choice)
        } else {
            Err(choice)
        }
    }

    pub fn from_choice(choice: String) -> Player {
        let validated_choice: String =
            Player::validate_choice(choice).expect("Bad Choice!");
        Player {
            choice: validated_choice,
        }
    }

    pub fn from_random() -> Player {
        let mut rng = rand::thread_rng();
        let random_choice =
            Player::get_choices().choose(&mut rng).unwrap().to_string();
        Player::from_choice(random_choice)
    }

    pub fn new(choice: String) -> Player {
        Player::from_choice(choice)
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
