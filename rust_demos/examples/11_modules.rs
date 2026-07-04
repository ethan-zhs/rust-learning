mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn square(n: i32) -> i32 {
        n * n
    }
}

mod text {
    pub fn shout(value: &str) -> String {
        value.to_uppercase()
    }
}

fn main() {
    println!("2 + 3 = {}", math::add(2, 3));
    println!("5 squared = {}", math::square(5));
    println!("{}", text::shout("rust modules"));
}
