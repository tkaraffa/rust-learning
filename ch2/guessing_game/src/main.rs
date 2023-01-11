use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Guess a number: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // if guess > secret {
        //     println!("Too big");
        // } else if guess < secret {
        //     println!("Too small");
        // } else {
        //     println!("Just right");
        //     break;
        // }

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("Just right.");
                break;
            }
        }
    }
}
