use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret_number {}",secret_number);
    loop{
        println!("Guess the Number!");
        println!("Please input your Number ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Faild to read line");
        let guess: u32 = guess.trim().parse().expect("Please input a number");
        println!("You guessed : {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small !"),
            Ordering::Greater => println!("Too big!"),
            Ordering:: Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

