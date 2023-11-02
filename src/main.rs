use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::ops::{Add, RangeInclusive, Sub};

// MutableRangeInclusive
#[derive(Clone, Copy)]
struct MutableRangeInclusive<Idx> {
    start: Idx,
    end: Idx,
}

impl<Idx: PartialOrd> MutableRangeInclusive<Idx> {
    fn contains(self, item: Idx) -> bool {
        item >= self.start && item <= self.end
    }
}

// Implement Length for RangeInclusive and MutableRangeInclusive
trait Length<T> {
    fn length(&self) -> T;
}

impl<T: Sub<Output = T> + Add<Output = T> + From<u8> + Copy> Length<T> for RangeInclusive<T> {
    fn length(&self) -> T {
        *self.end() - *self.start() + 1.into()
    }
}

impl<T: Sub<Output = T> + Add<Output = T> + From<u8> + Copy> Length<T>
    for MutableRangeInclusive<T>
{
    fn length(&self) -> T {
        self.end - self.start + 1.into()
    }
}

// MutableRangeInclusive from RangeInclusive
impl<Idx: Copy> From<RangeInclusive<Idx>> for MutableRangeInclusive<Idx> {
    fn from(range: RangeInclusive<Idx>) -> MutableRangeInclusive<Idx> {
        return MutableRangeInclusive {
            start: *range.start(),
            end: *range.end(),
        };
    }
}

fn main() {
    // The range of numbers that can be the secret number
    let numbers: RangeInclusive<u32> = 1..=100;

    // The range of possible numbers that might be the secret
    let mut possible_numbers: MutableRangeInclusive<u32> = numbers.clone().into();

    // Create the secret number the player has to guess
    let secret_number: u32 = rand::thread_rng().gen_range(numbers.clone());

    // The amount of tries the player has
    // Automatically calculated based on how many iterations a binary search worse case takes
    // Sorry lots of type casting nonsense, because integers can't use logarithms
    let mut tries: u32 = (numbers.length() as f32).log2() as u32 + 1;
    
    println!("Guess the number!");

    loop {
        // Calculate the mathematical best guess
        let best_guess = (possible_numbers.start + possible_numbers.end) / 2;

        println!(
            "You have {tries} {0} left.",
            if tries != 1 { "tries" } else { "try" }
        );

        // Collect player input
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the player input to a u32
        // If the player inputs `quit`, then break
        // If the player inputs `bot`, automatically make the best guess for them
        // If the player inputs `best`, print the best guess and continue
        // If the player inputs `secret`, print the secret number
        let guess: u32 = match guess.trim() {
            "bot" => best_guess,
            "quit" => break,
            "best" => {
                println!("The best guess is {best_guess}");
                continue;
            }
            "secret" => {
                println!("The secret number is {secret_number}");
                continue;
            }
            _ => match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            },
        };

        println!("You guessed {guess}");

        // If the player made the best guess, tell them that
        if best_guess == guess {
            println!("That was the best guess, good job!");
        }
        // If guess wasn't even in the range of possible numbers
        // tell the player they're a stupid idiot
        else if !possible_numbers.contains(guess) {
            println!("That was a crappy guess!");
        }
        // If it was an ok guess but it wasn't the best
        // tell the player what he should've done
        else {
            println!("The best guess was {best_guess}");
        }

        // Check if the guess was too small, too big, or if it was correct
        // then update the range of possible numbers
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");

                if possible_numbers.start <= guess {
                    possible_numbers.start = guess + 1;
                }
            }
            Ordering::Greater => {
                println!("Too big!");

                if possible_numbers.end >= guess {
                    possible_numbers.end = guess - 1;
                }
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        tries -= 1;

        // If the player ran out of guesses, end the game and tell them they lost
        if tries <= 0 {
            println!("You ran out of tries. The secret number was {secret_number}");
            break;
        }

        // Just print a gap for aesthetic reasons
        println!();
    }
}
