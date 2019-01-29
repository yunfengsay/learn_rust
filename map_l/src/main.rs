use std::collections::HashMap;
fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    println!("scores {:?}",scores);
    
    let teams = vec![String::from("blue"), String::from("yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

}
