use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum AppError {
    EmptyName,
    InvalidAge(i32),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::EmptyName => write!(f, "name cannot be empty"),
            AppError::InvalidAge(age) => write!(f, "invalid age: {age}"),
        }
    }
}

impl Error for AppError {}

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

fn create_user(name: &str, age: i32) -> Result<User, AppError> {
    if name.trim().is_empty() {
        return Err(AppError::EmptyName);
    }

    if !(0..=150).contains(&age) {
        return Err(AppError::InvalidAge(age));
    }

    Ok(User {
        name: name.trim().to_string(),
        age: age as u8,
    })
}

fn main() {
    for (name, age) in [("Alice", 18), ("", 20), ("Bob", 200)] {
        match create_user(name, age) {
            Ok(user) => println!("created user: {} ({})", user.name, user.age),
            Err(error) => println!("error: {error}"),
        }
    }
}
