trait Summary {
    fn summary(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summary(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

fn first<T>(items: &[T]) -> Option<&T> {
    items.first()
}

fn print_summary(item: &impl Summary) {
    println!("{}", item.summary());
}

fn main() {
    let numbers = vec![10, 20, 30];
    if let Some(n) = first(&numbers) {
        println!("first number = {n}");
    }

    let article = Article {
        title: String::from("Learning Rust"),
        author: String::from("Alice"),
    };
    print_summary(&article);
}
