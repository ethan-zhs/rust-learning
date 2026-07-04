#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    active: bool,
}

impl User {
    fn new(name: &str, age: u8) -> Self {
        Self {
            name: name.to_string(),
            age,
            active: true,
        }
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn main() {
    let mut user = User::new("Alice", 18);
    println!("{user:?}");
    println!("{} adult? {}", user.name, user.is_adult());

    user.deactivate();
    println!("after deactivate: {user:?}");
}
