enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn len(&self) -> usize {
        match self {
            List::Cons(_, next) => 1 + next.len(),
            List::Nil => 0,
        }
    }

    fn sum(&self) -> i32 {
        match self {
            List::Cons(value, next) => value + next.sum(),
            List::Nil => 0,
        }
    }
}

fn main() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    println!("len = {}", list.len());
    println!("sum = {}", list.sum());
}
