use std::convert::TryFrom;

#[derive(Debug)]
struct UserName(String);

impl From<&str> for UserName {
    fn from(value: &str) -> Self {
        Self(value.trim().to_string())
    }
}

#[derive(Debug)]
struct Age(u8);

impl TryFrom<i32> for Age {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if (0..=150).contains(&value) {
            Ok(Self(value as u8))
        } else {
            Err(format!("invalid age: {value}"))
        }
    }
}

fn main() {
    let name = UserName::from(" Alice ");
    println!("name = {}", name.0);

    let age = Age::try_from(18);
    println!("age = {:?}", age.map(|age| age.0));

    let bad_age = Age::try_from(200);
    println!("bad age = {bad_age:?}");
}
