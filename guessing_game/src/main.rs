extern crate rand;

//use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let mut guesses = 0;
    let mut high = 1000;
    let mut low = 1;

    let secret_number = rand::thread_rng().gen_range(low, high);

    loop {
        println!("Please guess a number from 1 to 100.");


        //let mut guess = String::new();

        //io::stdin().read_line(&mut guess)
        //    .expect("Failed to read line");

        //let guess: u32 = match guess.trim().parse() {
        //    Ok(num) => num,
        //    Err(_) => continue,
        //};


        let guess = (high - low) / 2 + low;

        println!("you guessed: {} with high of {} and low of {}", guess, high, low);
        guesses += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("To small!");
                low = guess;
            }

            Ordering::Greater => {
                println!("To big!");
                high = guess;
            }

            Ordering::Equal => {
                println!("You win with score {}!", guesses);
                break;
            }
        }
    }
}
