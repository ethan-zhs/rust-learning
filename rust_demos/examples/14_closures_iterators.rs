fn main() {
    let base = 10;
    let add_base = |x| x + base;
    println!("add_base(5) = {}", add_base(5));

    let numbers = vec![1, 2, 3, 4, 5, 6];

    let doubled_even_numbers: Vec<i32> = numbers
        .iter()
        .filter(|n| **n % 2 == 0)
        .map(|n| *n * 2)
        .collect();

    println!("doubled even numbers = {doubled_even_numbers:?}");

    let sum = numbers.iter().fold(0, |acc, n| acc + n);
    println!("sum = {sum}");
}
