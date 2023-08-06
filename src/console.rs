use std::io;


pub fn start_console(){
loop {
        println!("Press ^C to terminate the program...");
        let mut input = String::new();
        if input == "quit" {
            break;
        } else {
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

        }
    }
}
pub fn println<T: ToString>(message: T) {
    println!("[CONSOLE]: {}", message.to_string());
}
