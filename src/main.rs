use std::sync::{Arc, Mutex};
use std::thread;

struct Counter {
    value: Mutex<usize>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            value: Mutex::new(0),
        }
    }

    fn increment(&self) {
        let mut val = self.value.lock().unwrap();
        *val += 1;
    }

    fn get_value(&self) -> usize {
        let val = self.value.lock().unwrap();
        *val
    }
}

fn main() {
    let counter = Arc::new(Counter::new());
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.increment();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter value: {}", counter.get_value());
}
