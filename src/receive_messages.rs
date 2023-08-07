

use std::io::Read;
use std::net::TcpStream;

use colored::Color;

use crate::standard_console::LocalConsole;


// Import the external custom modules
// use crate::console::Console; // Make sure to adjust the path as needed


pub fn receive_messages(mut stream: TcpStream,) {

    let mut buffer = [0; 1024];
    let console = LocalConsole::new(module_path!(), Color::Green);


    console.console_println(format!("receive_messages thread:  @@@@@@@@@@@@@@@@@@@@@"));

    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    // End of stream, the server closed the connection
                    console.console_println(format!("Connection closed by the server"));
                    break;
                }
                let message = std::str::from_utf8(&buffer[..bytes_read])
                    .expect("Failed to parse message");
                console.console_println(format!("Received: {}", message));
            }
            Err(e) => {
                console.console_println(format!("Error reading from stream: {}", e));
                break;
            }
        }
    }
}


