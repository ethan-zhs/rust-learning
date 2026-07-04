pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn detect_even_number() {
        assert!(is_even(8));
        assert!(!is_even(7));
    }
}
