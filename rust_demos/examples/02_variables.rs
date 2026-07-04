fn main() {
    let language = "Rust";
    println!("language = {language}");

    let mut count = 1;
    count += 1;
    println!("count = {count}");

    let value = "42";
    let value: i32 = value.parse().expect("number text");
    println!("shadowed value + 1 = {}", value + 1);

    const MAX_SCORE: u32 = 100;
    println!("MAX_SCORE = {MAX_SCORE}");
}
