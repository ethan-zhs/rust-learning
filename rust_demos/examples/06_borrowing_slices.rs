fn length(text: &String) -> usize {
    text.len()
}

fn add_suffix(text: &mut String) {
    text.push_str(" is great");
}

fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(index) => &text[..index],
        None => text,
    }
}

fn main() {
    let text = String::from("Rust language");
    println!("length = {}", length(&text));
    println!("text is still usable: {text}");

    let mut slogan = String::from("Rust");
    add_suffix(&mut slogan);
    println!("{slogan}");

    let word = first_word(&slogan);
    println!("first word = {word}");
}
