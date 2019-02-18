fn main() {}

#[cfg(test)]

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[test]
fn test_2() {
    println!("test_2 {}", 23);
}

#[test]
fn another() {
    panic!("Make this test fail");
}
