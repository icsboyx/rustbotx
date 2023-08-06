use std::io::{self, Write};

pub fn start_console() {
    loop {
        print_prompt();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Process the user input here
                process_input(&input.trim());
            }
            Err(error) => eprintln!("Error reading input: {}", error),
        }
    }
}

pub fn print_prompt() {
    print!("################# "); // Display the prompt
    io::stdout().flush().unwrap();
}

pub fn process_input(input: &str) {
    match input {
        "quit" => {
            println!("Goodbye!");
            std::process::exit(0);
        }
        "hello" => {
            println!("Hello, User!");
        }
        _ => {
            println!("Invalid command: {}", input);
        }
    }
}
