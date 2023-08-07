
use crate::send_messages;
use std::io::{self, Write};
use std::net::TcpStream;

pub fn read_user_input(_stream: TcpStream) {
    // loop {
    //     println!("Press ^C to terminate the program...");
    //     let mut input = String::new();
    //     if input == "quit" {
    //         break;
    //     } else {
    //         io::stdin()
    //             .read_line(&mut input)
    //             .expect("Failed to read line");

    //         send_messages::send_messages(&stream, &&*"icsboyx", input);
    //     }
    // }

    // send_messages::send_messages(&stream, &&*"icsboyx", String::from("test"));
    
}
