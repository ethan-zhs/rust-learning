struct Counter {
    current: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Self { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current <= self.max {
            Some(self.current)
        } else {
            None
        }
    }
}

fn main() {
    let values: Vec<u32> = Counter::new(5)
        .filter(|n| n % 2 == 1)
        .map(|n| n * 10)
        .collect();

    println!("values = {values:?}");
}
