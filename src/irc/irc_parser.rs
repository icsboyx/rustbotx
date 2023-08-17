use itertools::Itertools;

use crate::c_println;

#[derive(Debug)]
pub struct IRCMessage {
    pub server_command: String,
    pub server_operation: ServerOperation,
    pub message: String,
}
#[derive(Debug)]
pub struct ServerOperation {
    pub sender: String,
    pub operation: String,
    pub receiver: String,
}


pub fn irc_parse_message(raw_message: String) -> IRCMessage {

    c_println!(format!("{} {}", "[RX]".green().bold(), raw_message));
    let message_split: Vec<String> = raw_message
        .splitn(3, ':')
        .map(|s| s.to_string())
        .pad_using(3, |_| "".to_string())
        .collect();
    
    let irc_message = IRCMessage {
        server_command: message_split[0].to_string(),
        server_operation: ServerOperation {
            sender: message_split[1]
                .split(' ').next()
                .unwrap_or(&message_split[1])
                .to_string(),
            operation: message_split[1].split(' ').nth(1).unwrap_or("").to_string(),
            receiver: message_split[1].split(' ').nth(2).unwrap_or("").to_string(),
        },
        message: message_split[2].to_string(),
    };
    c_println!(format!("{} {:#?}", "[RX]".green().bold(), irc_message));
    irc_message
}


