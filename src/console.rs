// console.rs

use colored::*;

pub struct Console;

impl Console {
    pub fn console_start(&self) {
        self.console_println("Console started");
    }

    pub fn console_println<S: AsRef<str>>(&self, message: S) {
        let message_str: &str = message.as_ref();
        println!("{} {}", "[CONSOLE]:".blue().bold(), message_str);
    }
}
