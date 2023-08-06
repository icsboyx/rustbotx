#![allow(dead_code)]
use std::io::Write;
use std::net::TcpStream;

pub fn send_messages(mut stream: &TcpStream, channel: &&str, message: String) {
    // This function sends a sample message to the specified channel.
    let payload: String = format!("PRIVMSG #{} :{}\r\n", channel, message);
    stream
        .write_all(payload.as_bytes())
        .expect("Failed to write to stream");
    println!("Sent: {}", message);
}