fn main() {
    println!("Hello, world!");
    test_func();
}

fn test_func() {
    let x = 5;
    println!("The value of x is: {:?}", x);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}