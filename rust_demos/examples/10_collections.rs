use std::collections::HashMap;

fn main() {
    let mut numbers = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("numbers = {numbers:?}");

    let mut text = String::from("hello");
    text.push(',');
    text.push_str(" rust");
    println!("text = {text}");

    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 90);
    scores.insert(String::from("Bob"), 82);

    for (name, score) in &scores {
        println!("{name}: {score}");
    }

    let alice_score = scores.get("Alice").copied().unwrap_or(0);
    println!("Alice score = {alice_score}");
}
