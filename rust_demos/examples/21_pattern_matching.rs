#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

fn main() {
    let value = Some(3);

    if let Some(n) = value {
        println!("if let value = {n}");
    }

    let mut stack = vec![1, 2, 3];
    while let Some(n) = stack.pop() {
        println!("pop = {n}");
    }

    let point = (10, 20);
    let (x, y) = point;
    println!("x = {x}, y = {y}");

    let user = User {
        name: String::from("Alice"),
        age: 18,
    };

    let User { name, age } = user;
    println!("{name} is {age}");

    let score = 86;
    match score {
        90..=100 => println!("great"),
        60..=89 => println!("pass"),
        _ => println!("try again"),
    }
}
