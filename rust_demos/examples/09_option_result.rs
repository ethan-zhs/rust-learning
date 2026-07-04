fn find_user(id: u32) -> Option<&'static str> {
    if id == 1 {
        Some("Alice")
    } else {
        None
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match find_user(1) {
        Some(name) => println!("found user: {name}"),
        None => println!("user not found"),
    }

    match find_user(2) {
        Some(name) => println!("found user: {name}"),
        None => println!("user not found"),
    }

    for (a, b) in [(10, 2), (10, 0)] {
        match divide(a, b) {
            Ok(value) => println!("{a} / {b} = {value}"),
            Err(error) => println!("error: {error}"),
        }
    }
}
