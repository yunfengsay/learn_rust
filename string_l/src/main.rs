fn main() {
    let s1 = String::from("abc ");
    let s2 = "def".to_string();
    let s3 = s1 + &s2;
    println!("s1 + s2 = {}", s3);
}
