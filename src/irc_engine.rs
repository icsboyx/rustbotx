
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;



use itertools::Itertools;

use crate::c_println;
use crate::startup_config::Config;

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



pub fn irc_connect(config: Config) {
    let server = "irc.chat.twitch.tv";
    let port = 6667;
    


    // Establish a TCP connection to the server
    let tcp_stream = TcpStream::connect(format!("{}:{}",server,port)).unwrap_or_else(|error| panic!("Failed to connect to server: {:?}", error));
    let mut tcp_stream_rx = tcp_stream.try_clone().unwrap();
    let mut tcp_stream_tx = tcp_stream.try_clone().unwrap();

    // Crate the IRC send receive  Queue
    let (irc_sender, irc_receiver) = channel::<String>();
    let irc_sender_server_message_actions = irc_sender.clone();
    
    
    let server_message_actions = move |payload: IRCMessage| {
        // Action Parser
        match payload.server_command.as_str() {
            "PING " => {
                let reply = format!("PONG :{}", payload.server_operation.sender);
                irc_sender_server_message_actions.send(reply).unwrap();
            }
            "" => {},
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
                irc_sender_server_message_actions.send(reply).unwrap();
            }
            "" => {},
            _ => {}
        };
    };

    let parse_server_messages = move |raw_message: &str| {
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
        server_message_actions(irc_message)
    };

   
    let irc_rx = thread::Builder::new().name("irc_rx".into());
    irc_rx
        .spawn(move || {
            // Read and process messages from the server
            let mut buffer = [0u8; 1024];
            loop {
                let bytes_read = tcp_stream_rx.read(&mut buffer).expect("non ok");
                if bytes_read == 0 {
                    // Connection closed by the server
                    break;
                }

                // Process the received data here (parsing messages, handling commands, etc.)
                let received_data = String::from_utf8_lossy(&buffer[..bytes_read]);
                let received_data_str = received_data.as_ref();
                parse_server_messages(received_data_str);
            }
        })
        .expect("Unable to spawn thread irc_rx");


    let irc_tx = thread::Builder::new().name("irc_tx".into());
    irc_tx
        .spawn(move || loop {
            let tx_payload = irc_receiver.try_iter().next();
            if let Some(tx_payload) = tx_payload {
                tcp_stream_tx
                    .write_all(tx_payload.as_bytes())
                    .unwrap();
                c_println!(format!("{} {}", "[TX]".cyan().bold(), tx_payload));
            }
            thread::sleep(Duration::from_millis(100));
        })
        .expect("Unable to spawn thread irc_tx");

    let pass_message = format!("PASS oauth:{}\r\n", config.token);
    let nick_message = format!("NICK {}\r\n", config.nickname);
    // let irc_options = format!("CAP REQ :twitch.tv/commands\r\nCAP REQ :twitch.tv/membership\r\nCAP REQ :twitch.tv/tags\r\n");
    let irc_options = "CAP REQ :twitch.tv/commands\r\n".to_string();
    // let irc_options = "CAP REQ :twitch.tv/membership\r\n".to_string();
    // let irc_options = "CAP REQ :twitch.tv/tags\r\n".to_string();

    // let irc_tx_clone = irc_tx_queue_arc_mutex.clone();

    irc_sender.send(pass_message).unwrap();
    irc_sender.send(nick_message).unwrap();
    irc_sender.send(irc_options).unwrap();

    for channel in config.channels {
        let join_message = format!("JOIN #{}\r\n", channel);
        irc_sender.send(join_message).unwrap();
    }
}