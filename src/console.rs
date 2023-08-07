// console.rs

use colored::*;

pub struct Console {
    message_queue: Vec<String>,
}

impl Console {
    pub fn new() -> Self {
        Console {
            message_queue: Vec::new(),
        }
    }

    pub fn console_start(&mut self) {
        self.queue_console_message("Console started");
        self.process_queue();
    }

    pub fn console_println<S: AsRef<str>>(&mut self, message: S) {
        let message_str: String = format!("{}{}", "[CONSOLE]:".blue().bold(),  message.as_ref());
        self.queue_console_message(message_str);
        self.process_queue();
    }

    fn queue_console_message<S: AsRef<str>>(&mut self, message: S) {
        self.message_queue.push(String::from(message.as_ref()));
    }

    fn process_queue(&mut self) {
        while let Some(message) = self.message_queue.pop() {
            println!("{}", message);
        }
    }
}
