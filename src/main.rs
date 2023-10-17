use rand::distributions::Alphanumeric;
use random_word::Lang;
use std::cmp::Ordering;
use std::io;

enum Guess<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let secret_number = generate_secret_number();
    //println!("The secret number is: {}", secret_number);

    loop {
        let attempts = 3;
        match guess_secret_number(attempts) {
            Guess::Ok(num) => {
                //println!("You guessed: {}", num);
                match num.cmp(&secret_number) {
                    Ordering::Less => println!("Too small!"),
                    Ordering::Greater => println!("Too big!"),
                    Ordering::Equal => {
                        println!("You win!");
                        break;
                    }
                }
            }
            Guess::Err(msg) => {
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

fn generate_secret_number() -> u8 {
    let secret_number = rand::thread_rng().gen_range(42..=100);
    //println!("The secret number is: {}", secret_number);
    secret_number
}

fn guess_secret_number(mut attempts: u8) -> Guess<u8, String> {
    let mut failed_attempts: Vec<String> = Vec::new();

    loop {
        if attempts == 0 {
            return Guess::Err(format!(
                "You have no more attempts! Failed attempts: {:?}",
                failed_attempts
            ));
        } else {
            attempts = attempts - 1;
        }

        println!("Please input your guess:");
        let mut guess = String::new();

        // read a line from stdin
        match io::stdin().read_line(&mut guess) {
            Ok(_) => {
                //print!("You guessed: {guess}");
            }
            Err(error) => {
                println!("Failed to read line: {}", error);
                continue;
            }
        }

        // attempt to parse the input as an integer
        match guess.trim().parse::<u8>() {
            Ok(num) => return Guess::Ok(num),
            Err(_) => {
                println!("Please type a number!");
                failed_attempts.push(guess.trim().to_string());
                continue;
            }
        }
    }
}

/// Sorts a mutable slice of integers in ascending order.
///
/// # Arguments
///
/// * `list` - A mutable reference to a slice of integers that needs to be sorted.
///
/// # Examples
///
/// ```
/// let mut list = [5, 2, 8, 1, 9];
/// sort2(&mut list);
/// assert_eq!(list, [1, 2, 5, 8, 9]);
/// ```
fn sort2(list: &mut [i32]) {
    list.sort();
}

// a function that build a list of random  strings
fn generate_random_strings(count: usize, min: usize, max: usize) -> Vec<String> {
    let mut list = Vec::with_capacity(count);
    for _ in 0..count {
        let random_length = rand::thread_rng().gen_range(min..=max);
        let random_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(random_length)
            .map(char::from)
            .collect();
        list.push(random_string);
    }
    list
}

/// Generates a list of random words based on the specified language.
///
/// # Arguments
///
/// * `count` - The number of random words to generate.
/// * `lang` - The language of the random words to generate.
///
/// # Returns
///
/// A vector containing `count` random words based on the specified language.
fn generate_random_words(count: usize, lang: Lang) -> Vec<String> {
    (0..count)
        .map(|_| random_word::gen(lang).to_string())
        .collect()

    // let mut list = Vec::new();
    // for _ in 0..count {
    //     let random_word = random_word::gen(lang).to_string();
    //     list.push(random_word);
    // }
    // list
}

use rand::Rng;

/// Generates a list of random numbers within a specified range.
///
/// # Arguments
///
/// * `count` - The number of random numbers to generate.
/// * `min` - The minimum value of the random numbers.
/// * `max` - The maximum value of the random numbers.
///
/// # Returns
///
/// A vector containing `count` random numbers between `min` and `max`.
fn generate_random_numbers(count: usize, min: i32, max: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut list = Vec::with_capacity(count);

    for _ in 0..count {
        let s = rng.gen_range(min..max);
        list.push(s);
    }

    list
}
