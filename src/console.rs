use colored::*;

pub fn start_console() {
   //To Do
}


pub fn console_println<S: AsRef<str>>(message: S) {
    let message_str: &str = message.as_ref();
    println!("{}{}", "[CONSOLE]: ".blue().bold(), message_str);
}
