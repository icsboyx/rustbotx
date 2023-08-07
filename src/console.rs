use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct ConsoleQueue {
    queue: Arc<Mutex<Vec<String>>>,
}

impl ConsoleQueue {
    pub fn new(queue: Arc<Mutex<Vec<String>>>) -> Self {
        ConsoleQueue { queue }
    }

    pub fn start(&self) {
        let monitor_queue = self.queue.clone();

        // Monitor thread
        thread::spawn(move || {
            let mut last_queue_len = 0; // Store the previous queue length
            loop {
                let queue = monitor_queue.lock().unwrap();
                let queue_len = queue.len();

                if queue_len > last_queue_len {
                    for item in queue.iter().skip(last_queue_len) {
                        println!("New item in queue: {:?}", item);
                    }
                    last_queue_len = queue_len;
                }

                thread::sleep(Duration::from_secs(2)); // Adjust the sleep duration as needed
            }
        });
    }
        pub fn console_println(&self, item: String) {
            self.queue.lock().unwrap().push(item);
        }

}