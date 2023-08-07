
use std::io::Read;
use std::net::TcpStream;
// Import the external custom modules
use crate::Console;


pub fn receive_messages(mut stream: TcpStream,) {
    let mut buffer = [0; 1024];
    loop {
        Console.console_println(format!("Waiting for data..."));
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    // End of stream, the server closed the connection
                    Console.console_println(format!("Connection closed by the server"));
                    break;
                }
                let message = std::str::from_utf8(&buffer[..bytes_read])
                    .expect("Failed to parse message");
                Console.console_println(format!("Received: {}", message));
            }
            Err(e) => {
                Console.console_println(format!("Error reading from stream: {}", e));
                break;
            }
        }
    }
}
