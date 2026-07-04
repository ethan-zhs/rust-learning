fn main() {
    let age: u32 = 18;
    let price: f64 = 19.9;
    let ok: bool = true;
    let letter: char = 'R';

    let point: (i32, i32) = (10, 20);
    let numbers: [i32; 3] = [1, 2, 3];

    println!("age = {age}");
    println!("price = {price}");
    println!("ok = {ok}");
    println!("letter = {letter}");
    println!("point = ({}, {})", point.0, point.1);
    println!("first number = {}", numbers[0]);
}
