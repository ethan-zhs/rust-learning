#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

struct Resource {
    name: String,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("drop resource: {}", self.name);
    }
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a;
    println!("a = {a:?}, b = {b:?}");
    println!("point sum = {}", a.x + a.y);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    let _resource = Resource {
        name: String::from("file handle"),
    };
    println!("resource will be dropped at the end of scope");
}
