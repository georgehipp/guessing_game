extern crate rand;

use std::cmp::Ordering;
use rand::Rng;
use std::io;

fn main() {
    game_logic(start_game());
}

fn start_game() -> u32 {
    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is: {}", secret_number);
    return secret_number;
}

fn game_logic(secret_number: u32) {
    let mut count: u32 = 0;
    loop {
        count = count + 1;
        println!("\nPlease Input your Guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        match guess.to_lowercase().as_ref() {
            "quit\n" => {
                println!("Thanks for playing!");
                break;
                },
            "exit\n" => {
                println!("Thanks for playing!");
                break;
                },
            _ => (),
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                guess.pop();
                println!("{} is not a positive whole number!", guess);
                continue;
            },
        };

        println!("You guessed: {}", guess);

        let result = guess.cmp(&100);
        if Ordering::Greater == result {
             println!("Too High! Guess a number value from 1 to 100!");
             continue;
        }

        let result = guess.cmp(&1);
        if Ordering::Less == result {
            println!("Too Low! Guess a number value from 1 to 100!");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                let guessdif = secret_number - guess;
                if guessdif > 50 {
                    println!("Way off, much higher!");
                    continue;
                }
                if guessdif > 25 {
                    println!("Much higher!");
                    continue;
                }
                if guessdif >= 10 {
                    println!("Higher!");
                    continue;
                }
                if guessdif < 10 {
                    println!("A little higher!");
                    continue;
                }
            },
            Ordering::Greater => {
                let guessdif = guess - secret_number;
                match guessdif {
                    1 ... 10 => {
                        println!("A little lower!");
                        continue;
                    }
                    11 ... 25 => {
                        println!("Lower!");
                        continue;
                    }
                    26 ... 50 => {
                        println!("Much lower!");
                        continue;
                    }
                    _ => {
                        println!("Way off, much lower!");
                        continue;
                    }
                }
            },
            Ordering::Equal => {
                match count {
                    1 ... 3 => {
                        println!("{} tries, you must be psychic!", count);
                        break;
                    }
                    4 ... 5 => {
                        println!("{} tries, nice job!", count);
                        break;
                    }
                    6 ... 9 => {
                        println!("{} tries, don't quit your day job!", count);
                        break;
                    }
                    _ => {
                        println!("{} tries, you should be embarrassed!", count);
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secret_number_is_btw_0_100() {
        let test_secret_number = start_game();
        assert!(test_secret_number < 101 && test_secret_number > 0);
    }
}