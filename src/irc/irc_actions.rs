use std::sync::mpsc::Sender;

use super::irc_parser::IRCMessage;

pub fn irc_actions(tx_msg_push: Sender<String>, payload: IRCMessage) {
    // Action Parser
    match payload.server_command.as_str() {
        "PING " => {
            let reply = format!("PONG :{}", payload.server_operation.sender);
            // irc_sender_server_message_actions.send(reply).unwrap();
            tx_msg_push.send(reply).unwrap();
        }
        "" => {}
        _ => {}
    };
    match payload.message.to_lowercase().as_str() {
        "!ciao\r\n" => {
            let reply = format!(
                "PRIVMSG {} :{}@{}\r\n",
                payload.server_operation.receiver,
                "Ciao a te ",
                payload.server_operation.sender.split('!').next().unwrap(),
            );
            tx_msg_push.send(reply).unwrap();
            // irc_sender_server_message_actions.send(reply).unwrap();
        }
        "" => {}
        _ => {}
    };
}
