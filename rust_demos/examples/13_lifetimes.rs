fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

struct Book<'a> {
    title: &'a str,
}

fn main() {
    let first = String::from("short");
    let second = String::from("a much longer string");
    let result = longest(&first, &second);
    println!("longest = {result}");

    let title = String::from("The Rust Book");
    let book = Book { title: &title };
    println!("book title = {}", book.title);
}
