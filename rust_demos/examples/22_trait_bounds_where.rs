use std::fmt::Display;

fn print_twice<T: Display>(value: T) {
    println!("{value}");
    println!("{value}");
}

fn compare_and_print<T>(a: T, b: T)
where
    T: Display + PartialOrd + Copy,
{
    println!("a = {a}, b = {b}");
    if a > b {
        println!("a is larger");
    } else {
        println!("a is not larger");
    }
}

fn largest<T>(items: &[T]) -> Option<T>
where
    T: PartialOrd + Copy,
{
    let mut best = *items.first()?;
    for item in items {
        if *item > best {
            best = *item;
        }
    }
    Some(best)
}

fn main() {
    print_twice("Rust");
    compare_and_print(10, 8);

    let numbers = [3, 8, 2, 5];
    println!("largest = {:?}", largest(&numbers));
}
