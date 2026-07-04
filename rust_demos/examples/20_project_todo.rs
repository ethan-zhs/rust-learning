#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Todo,
    Done,
}

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    status: Status,
}

impl Task {
    fn new(id: u32, title: &str) -> Self {
        Self {
            id,
            title: title.to_string(),
            status: Status::Todo,
        }
    }

    fn finish(&mut self) {
        self.status = Status::Done;
    }

    fn is_done(&self) -> bool {
        self.status == Status::Done
    }
}

fn print_tasks(tasks: &[Task]) {
    for task in tasks {
        let marker = if task.is_done() { "x" } else { " " };
        println!("[{marker}] #{} {}", task.id, task.title);
    }
}

fn main() {
    let mut tasks = vec![
        Task::new(1, "Install Rust"),
        Task::new(2, "Read ownership chapter"),
        Task::new(3, "Write a small CLI"),
    ];

    tasks[0].finish();

    println!("all tasks:");
    print_tasks(&tasks);

    let remaining: Vec<&Task> = tasks.iter().filter(|task| !task.is_done()).collect();
    println!("remaining count = {}", remaining.len());
}
