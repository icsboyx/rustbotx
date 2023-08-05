use std::io::Read;
use std::net::TcpStream;

pub fn receive_messages(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        println!("Waiting for data...");
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    // End of stream, the server closed the connection
                    println!("Connection closed by the server");
                    break;
                }
                let message = std::str::from_utf8(&buffer[..bytes_read])
                    .expect("Failed to parse message");
                println!("Received: {}", message);
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                break;
            }
        }
    }
}
