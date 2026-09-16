use std::cmp::Ordering;

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

        match guess.cmp(&modded_num) {
            
        }
    
    }
}
