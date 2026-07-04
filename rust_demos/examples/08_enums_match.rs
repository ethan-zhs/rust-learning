enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn describe_direction(direction: Direction) -> &'static str {
    match direction {
        Direction::Up => "up",
        Direction::Down => "down",
        Direction::Left => "left",
        Direction::Right => "right",
    }
}

fn handle_message(message: Message) {
    match message {
        Message::Quit => println!("quit"),
        Message::Move { x, y } => println!("move to ({x}, {y})"),
        Message::Write(text) => println!("write: {text}"),
    }
}

fn main() {
    println!("{}", describe_direction(Direction::Up));
    println!("{}", describe_direction(Direction::Down));
    println!("{}", describe_direction(Direction::Left));
    println!("{}", describe_direction(Direction::Right));

    handle_message(Message::Move { x: 3, y: 4 });
    handle_message(Message::Write(String::from("hello")));
    handle_message(Message::Quit);
}
