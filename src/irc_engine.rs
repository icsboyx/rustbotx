
use chrono::Local;
use std::{io::Write, net::TcpStream};

use crate::c_println;

// Example of received server message.
// PING :tmi.twitch.tv ==> Command :Sender
// Example of replay to server message.
// PONG :tmi.twitch.tv ==> Command :Sender
//
// Example of message received from channel
// :icsboyx!icsboyx@icsboyx.tmi.twitch.tv PRIVMSG #icsboyx :Ciao
// :sender MESSAGE_TYPE #channel:Message
#[derive(Debug)]
pub struct IRCMessage {
    pub server_command: String,
    pub server_operation: ServerOperation,
    pub message: String,
}
#[derive(Debug)]
pub struct ServerOperation {
    pub sender: String,
    pub message_type: String,
    pub context: String,
}

pub fn irc_server_messages_engine(stream: &TcpStream, payload: IRCMessage) {
    c_println!(format!("{:#?}", payload));
    match (payload.server_command.as_str(),payload.message.is_empty()) {
        ("PING ",true) => {
            c_println!("Received a PING command!".yellow().bold());
            let reply = format!("PONG :{}\r\n", payload.server_operation.sender);
            send_server_message(stream, &reply);

        }
        ("!Ciao",true) => {
            c_println!("Received a !Ciao command!".yellow().bold());
        }
        _ => {
            //to do 
        }
    };
    match payload.message.to_lowercase().as_str() {
        "ciao" =>{
            let message = format!("@{} Hello to you my friend! ", payload.server_operation.sender.split(':').nth(1).unwrap().split('!').next().unwrap());
            let reply = format!("{} {} :{}\r\n",payload.server_operation.message_type, payload.server_operation.context, message);
            println!("Ciao Found!");
            send_server_message(stream, &reply);
        }
        "!time"=>{
        let message = Local::now().format("%H:%M:%S").to_string();
        let reply = format!("{} {} :{}\r\n",payload.server_operation.message_type, payload.server_operation.context, message);
        send_server_message(stream, &reply);
        }
        _=>{
            //do do
        }
    };
}

pub fn send_server_message(mut stream: &TcpStream, payload: &str) {
    // This function sends a sample message to server.
    c_println!(format!("{}{:#?}", "[Re:]".green(),payload));
    stream
        .write_all(payload.as_bytes())
        .expect("Failed to write to stream");
}


pub fn parse_server_messages(stream: &TcpStream, message: &str){

    let part_a: Vec<String> = message.splitn(3,':').map(|s| s.to_string()).collect();
    let part_b: Vec<String> = message.split(' ').map(|s| s.to_string()).collect();
    let payload = IRCMessage {
        server_command: if !part_a.is_empty() { part_a[0].clone() } else { "".to_string() },
        server_operation: ServerOperation {
            sender: if !part_b.is_empty() { part_b[0].clone() } else { "".to_string() },
            message_type: if part_b.len() > 1 { part_b[1].clone() } else { "".to_string() },
            context: if part_b.len() > 2 { part_b[2].clone() } else { "".to_string() },
        },
        message: if part_a.len() > 2 { part_a[2].clone() } else { "".to_string() },
    };
    irc_server_messages_engine(stream, payload);
}

