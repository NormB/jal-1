use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    match io::stdin().read_line(&mut guess) {
        Ok(_) => {},
        Err(error) => panic!("Failed to read line: {}", error),
    }

    println!("You guessed: {}", guess);
}
