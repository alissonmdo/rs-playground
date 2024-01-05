use console::{Style, Term};
use rand::Rng;
use std::{cmp::Ordering, io:: {self, Write}, thread, time::Duration};

pub fn main() {
    let term = Term::stdout();
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts: u32 = 0;
    loop {
        print!("Please input your number: ");
        let _ = io::stdout().flush();
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You didn't input a number...\n");
                continue;
            }
        };
        attempts+= 1;
        term.clear_screen().expect("Failed to clear screen");
        println!("You guessed: {}", Style::new().cyan().apply_to(guess));
        thread::sleep(Duration::from_millis(200));
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!(
                    "It's {}, try again!",
                    Style::new().red().apply_to("too small")
                )
            }
            Ordering::Greater => {
                println!(
                    "It's {}, try again!",
                    Style::new().red().apply_to("too big")
                )
            }
            Ordering::Equal => {
                println!(
                    "{} It took {} attempts.",
                    Style::new().green().apply_to("You win!"),
                    Style::new().bold().apply_to(attempts.to_string())
                );
                break;
            }
        }
    }
}
