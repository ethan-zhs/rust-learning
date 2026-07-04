use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut value = counter.lock().expect("mutex poisoned");
            *value += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("thread failed");
    }

    println!("counter = {}", *counter.lock().expect("mutex poisoned"));
}
