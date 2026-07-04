trait SimpleIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Countdown {
    current: u32,
}

impl SimpleIterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == 0 {
            None
        } else {
            let value = self.current;
            self.current -= 1;
            Some(value)
        }
    }
}

fn main() {
    let mut countdown = Countdown { current: 3 };

    while let Some(value) = countdown.next() {
        println!("{value}");
    }
}
