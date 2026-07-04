use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let boxed = Box::new(5);
    println!("boxed = {boxed}");

    let shared = Rc::new(String::from("shared text"));
    let shared_a = Rc::clone(&shared);
    let shared_b = Rc::clone(&shared);
    println!("{} / {}", shared_a, shared_b);
    println!("reference count = {}", Rc::strong_count(&shared));

    let value = RefCell::new(10);
    *value.borrow_mut() += 5;
    println!("RefCell value = {}", *value.borrow());
}
