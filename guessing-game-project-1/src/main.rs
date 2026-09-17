use std::cmp::Ordering;
use std::io;

fn main() {


    let num = rand::random::<u8>();
    let modded_num = num % 101;

    let output = format!("number is {modded_num}");


    println!("{}", output);

    loop {
        println!("Welcome to the guessing game! Guess a number from 0 to 100.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Reading line failed.");

        match guess.trim().parse::<u8>() {
            Ok(val) => {
                match val.cmp(&modded_num) {
                    Ordering::Less => println!("Too low!!!"),
                    Ordering::Equal => {
                        println!("Correct! Exiting...");
                        return
                    },
                    Ordering::Greater => println!("Too high!!!"),
                }
            },
            Err(_val) => {
                println!("Invalid Value. Try Again");
            },
        }
    }
}
