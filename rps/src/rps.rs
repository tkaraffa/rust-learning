use std::cmp::Ordering;

#[derive(Eq)]
pub struct Player {
    choice: String,
}

impl Player {
    pub fn new(choice: String) -> Player {
        let string_choice: &str = &choice;
        let validated_choice: String = match string_choice {
            "rock" | "paper" | "scissors" => choice,
            _ => panic!("bad choice"),
        };

        Player {
            choice: validated_choice,
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
        // <
        ((self.choice == "scissors") & (other.choice == "rock"))
            | ((self.choice == "rock") & (other.choice == "paper"))
            | ((self.choice == "paper") & (other.choice == "scissors"))
    }

    fn gt(&self, other: &Self) -> bool {
        // >
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
