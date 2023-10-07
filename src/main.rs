use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess:");

    let mut guess = String::new();

    // read a line from stdin

    match io::stdin().read_line(&mut guess) {
        Ok(_) => {
            print!("You guessed: {guess}");
        }
        Err(error) => panic!("Failed to read line: {}", error),
    }

    println!("You guessed: {}", guess);

    // create a lit of integers
    let mut list = [42, 12, 1, 100, 15, 53, 22, 11, 1, 42];
    let sorted_list = sort(&list);
    println!("Sorted list: {:?}", sorted_list);
    sort2(&mut list);
    println!("Updated list: {:?}", list);
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
