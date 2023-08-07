
use std::io::Read;
use std::net::TcpStream;
use std::collections::VecDeque;
// Import the external custom modules
use crate::ConsoleQueue;


pub fn receive_messages(mut stream: TcpStream,console_queue: ConsoleQueue) {
    let mut buffer = [0; 1024];
    loop {
        console_queue.console_println(format!("Waiting for data..."));
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    // End of stream, the server closed the connection
                    console_queue.console_println(format!("Connection closed by the server"));
                    break;
                }
                let message = std::str::from_utf8(&buffer[..bytes_read])
                    .expect("Failed to parse message");
                console_queue.console_println(format!("Received: {}", message));
            }
            Err(e) => {
                console_queue.console_println(format!("Error reading from stream: {}", e));
                break;
            }
        }
    }
}
