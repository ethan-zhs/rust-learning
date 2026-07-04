use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    for id in 1..=3 {
        let tx = tx.clone();
        thread::spawn(move || {
            let message = format!("message from worker {id}");
            tx.send(message).expect("send message");
        });
    }

    drop(tx);

    for message in rx {
        println!("{message}");
    }
}
