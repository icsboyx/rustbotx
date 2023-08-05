use std::io;
use std::net::TcpStream;
use crate::send_messages;

pub fn read_user_input(stream: TcpStream) {
    loop {
        println!("Press ^C to terminate the program...");
        let mut input = String::new();
        if input == "quit" {
            break;
        } else {
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            send_messages::send_messages(&stream, &&*"profandreapollini", input);
        }
    }
}
