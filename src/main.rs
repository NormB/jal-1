use rand::{distributions::Alphanumeric, Rng};
use random_word::Lang;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = generate_secret_number();
    println!("The secret number is: {}", secret_number);

    let guess = guess_secret_number();
    println!("You guessed: {}", guess);

    // create a list of random numbers
    let mut rng = rand::thread_rng();
    let mut list = [0; 10];
    for i in 0..10 {
        list[i] = rng.gen_range(0..100);
    }
    println!("Random list: {:?}", list);

    // create a lit of integers
    let sorted_list = sort(&list);
    println!("Sorted list: {:?}", sorted_list);
    sort2(&mut list);
    println!("Updated list: {:?}", list);

    let l = random_string_list();
    println!("Random string list: {:?}", l);

    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    println!("{}", s);

    let word = random_word::gen(Lang::En);
    println!("{}", word);
}

fn generate_secret_number() -> i32 {
    let secret_number = rand::thread_rng().gen_range(42..=100);
    println!("The secret number is: {}", secret_number);
    secret_number
}

fn guess_secret_number() -> i32 {
    loop {
        println!("Please input your guess:");
        let mut guess = String::new();

        // read a line from stdin
        match io::stdin().read_line(&mut guess) {
            Ok(_) => {
                print!("You guessed: {guess}");
            }
            Err(error) => {
                println!("Failed to read line: {}", error);
                continue;
            }
        }

        // attempt to parse the input as an integet
        match guess.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        }
    }
}

// create a function to sort a list of integers
fn sort(list: &[i32]) -> Vec<i32> {
    let mut v = list.to_vec();
    v.sort();
    v
}

// create a function to sort a list of integers by updating the list
fn sort2(list: &mut [i32]) {
    list.sort();
}

// create a function that build a list of random  strings
fn random_string_list() -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut list = Vec::new();
    for _ in 0..10 {
        let s: String = rng.gen_range(0..100).to_string();
        list.push(s);
    }
    list
}

// create a function that builds random dictionary words
