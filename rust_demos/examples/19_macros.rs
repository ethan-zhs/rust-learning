macro_rules! say {
    ($message:expr) => {
        println!("message: {}", $message);
    };
}

macro_rules! make_vec {
    ($($item:expr),* $(,)?) => {{
        let mut values = Vec::new();
        $(
            values.push($item);
        )*
        values
    }};
}

fn main() {
    say!("hello macro");

    let numbers = make_vec![1, 2, 3];
    println!("numbers = {numbers:?}");

    let text = format!("{} {}", "hello", "format macro");
    println!("{text}");
}
