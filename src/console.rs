use std::sync::{Arc, Mutex};
use std::thread;

pub struct Console {
    messages_queue: Arc<Mutex<Vec<String>>>,
    max_capacity: usize,
}

impl Console {
    pub fn new(max_capacity: usize) -> Self {
        Console {
            messages_queue: Arc::new(Mutex::new(Vec::new())),
            max_capacity,
        }
    }

    pub fn console_start(&self) {
        let queue_clone = self.messages_queue.clone();
        let max_capacity = self.max_capacity;

        thread::spawn(move || {
            loop {
                thread::sleep(std::time::Duration::from_secs(1));
                let mut messages = queue_clone.lock().unwrap();
                while messages.len() > max_capacity {
                    messages.remove(0);
                }
                for message in messages.iter() {
                    println!("{}", message);
                }
            }
        });
    }

    pub fn console_println(&self, message: &str) {
        let mut messages = self.messages_queue.lock().unwrap();
        if messages.len() >= self.max_capacity {
            messages.remove(0);
        }
        messages.push(message.to_string());
    }
}
