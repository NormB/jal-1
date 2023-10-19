use jal_1::generate_random_numbers;
use jal_1::generate_random_strings;
use jal_1::generate_random_words;
use jal_1::generate_secret_number;
use jal_1::guess_secret_number;
use jal_1::sort2;
use jal_1::Guess;
use jal_1::MyError;

use random_word::Lang;
use std::cmp::Ordering;

fn main() {
    let secret_number = generate_secret_number();
    //println!("The secret number is: {}", secret_number);

    loop {
        let valid_attempts = 3;
        match guess_secret_number(valid_attempts) {
            Guess::Ok(num) => {
                //println!("You guessed: {}", num);
                println!("valid_attempts: {}", valid_attempts);
                match num.cmp(&secret_number) {
                    Ordering::Less => println!("Too small!"),
                    Ordering::Greater => println!("Too big!"),
                    Ordering::Equal => {
                        println!("You win!");
                        break;
                    }
                }
            }
            Guess::Err(MyError::no_more_attempts(msg)) => {
                println!("{}", msg);
                break;
            }
            Guess::Err(MyError::read_error(msg)) => {
                println!("{}", msg)
            }
        }
    }

    let mut list = generate_random_numbers(25, 0, 100);
    println!("Random list: {:?}", list);

    sort2(&mut list);
    println!("Updated list (inplace): {:?}", list);

    let l = generate_random_strings(10, 1, 10);
    println!("Random string list: {:?}", l);

    let l = generate_random_words(10, Lang::En);
    println!("Random word list (En): {:?}", l);

    let l = generate_random_words(10, Lang::Fr);
    println!("Random word list (Fr): {:?}", l);
}
