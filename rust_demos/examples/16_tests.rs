fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    assert_eq!(add(2, 3), 5);
    assert!(is_even(8));
    assert!(!is_even(7));
    println!("all simple assertions passed");
}
