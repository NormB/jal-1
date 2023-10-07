fn main() {
    println!("Hello, world!");
    test_func();
}

fn test_func() {
    println!("Hello, jal-1");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}