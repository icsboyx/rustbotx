use crate::c_println;
use crate::irc::irc_actions::*;
use crate::irc::irc_parser::*;
use crate::startup_config::Config;
use colored::*;
use ctrlc::set_handler;
use openssl::ssl::{SslConnector, SslMethod, SslStream};
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{process, thread};

#[derive(Debug)]
enum StreamType {
    MyTcpStream(Arc<Mutex<TcpStream>>, Arc<Mutex<TcpStream>>),
    MySslStream(
        Arc<Mutex<SslStream<TcpStream>>>,
        Arc<Mutex<SslStream<TcpStream>>>,
    ),
}

pub fn irc_connect(config: Config) {
    // Create send queue
    let (tx_msg_push, tx_msg_pull) = mpsc::channel::<String>();

    // Create TcpStream.
    let tcp_stream = create_tcp(&config).unwrap();

    // Create Stream SSL or normal
    let create_streams = if !config.ssl_tls {
        create_tcp_streams(tcp_stream)
    } else {
        create_ssl_streams(tcp_stream, &config)
    };

    let rx_stream: Arc<Mutex<dyn Read + Send>>; // Define the receive stream
    let tx_stream: Arc<Mutex<dyn Write + Send>>; // Define the transmit stream

    match create_streams {
        StreamType::MyTcpStream(rx, tx) => {
            rx_stream = rx as Arc<Mutex<dyn Read + Send>>;
            tx_stream = tx as Arc<Mutex<dyn Write + Send>>;
        }
        StreamType::MySslStream(rx, tx) => {
            rx_stream = rx as Arc<Mutex<dyn Read + Send>>;
            tx_stream = tx as Arc<Mutex<dyn Write + Send>>;
        }
    }
    let rx_payload_tx_msg_push = tx_msg_push.clone();

    // Perform read operations on the SSL stream
    let _ = thread::Builder::new()
        .name("receiver_thread".to_string())
        .spawn(move || loop {
            let mut rx_payload = String::new();
            match rx_stream.lock().unwrap().read_to_string(&mut rx_payload) {
                Ok(n) if n == 0 => {
                    println!("Server connection is closed. Exiting...");
                    process::exit(1);
                }
                Ok(_) => {
                    println!("Place holder for connection.");
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                    } else {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            if !rx_payload.is_empty() {
                irc_actions(
                    rx_payload_tx_msg_push.clone(),
                    irc_parse_message(rx_payload),
                );
            }
            thread::sleep(Duration::from_millis(100));
        });

    // Perform write operations on the SSL stream
    let _ = thread::Builder::new()
        .name("sender_thread".to_string())
        .spawn(move || {
            while let Ok(tx_payload) = tx_msg_pull.recv() {
                tx_stream
                    .lock()
                    .unwrap()
                    .write_all(tx_payload.as_bytes())
                    .unwrap();
                c_println!(format!(" {}{}", "[TX]".cyan().bold(), tx_payload));
            }
        });

    irc_authentication(tx_msg_push, &config);

    // Register the CTRL+C signal handler
    set_handler(move || {
        println!(
            "{}",
            "\r\nCTRL+C signal received. Terminating..."
                .red()
                .underline()
        );
        std::process::exit(0);
    })
    .unwrap();

    loop {
        thread::sleep(Duration::from_secs(10));
    }
}

fn create_tcp(config: &Config) -> Result<TcpStream, io::Error> {
    // Combine server address and port into a single string
    let address = format!("{}:{}", config.server, config.port);
    // Connect to localhost:667 using a TcpStream.
    TcpStream::connect(address)
}

fn create_tcp_streams(tcp_stream: TcpStream) -> StreamType {
    tcp_stream.set_nonblocking(true).unwrap();
    let tcp_stream = Arc::new(Mutex::new(tcp_stream));
    let rx_stream = tcp_stream.clone();
    let tx_stream = tcp_stream.clone();
    StreamType::MyTcpStream(rx_stream, tx_stream)
}

#[allow(clippy::type_complexity)]
fn create_ssl_streams(tcp_stream: TcpStream, config: &Config) -> StreamType {
    // Create an OpenSSL SSL connector.
    let mut connector = SslConnector::builder(SslMethod::tls()).unwrap();
    if !config.ssl_verify_mode {
        connector.set_verify(openssl::ssl::SslVerifyMode::NONE);
    }
    let connector = connector.build();
    // Wrap the TcpStream in an SSL stream.
    let ssl_stream = connector
        .connect(config.server.as_str(), tcp_stream)
        .unwrap();
    // Get a Tcp stream out from the connector.
    let tcp_stream_after_connect = ssl_stream.get_ref();
    // Set a nonblocking for read operations (optional), Must be done here not before connect.
    tcp_stream_after_connect.set_nonblocking(true).unwrap();
    // Create a Arc Mutex for cloning the connector
    let ssl_stream = Arc::new(Mutex::new(ssl_stream));
    // Clone ssl_stream for tx and rx operations.
    let rx_stream = ssl_stream.clone();
    let tx_stream = ssl_stream.clone();
    StreamType::MySslStream(rx_stream, tx_stream)
}

fn irc_authentication(tx_msg_push: Sender<String>, config: &Config) {
    let pass_message = format!("PASS oauth:{}\r\n", config.token);
    tx_msg_push.send(pass_message).unwrap();

    let nick_message = format!("NICK {}\r\n", config.nickname);
    tx_msg_push.send(nick_message).unwrap();

    let irc_options = "CAP REQ :twitch.tv/commands\r\n".to_string();
    tx_msg_push.send(irc_options).unwrap();

    // let irc_options = "CAP REQ :twitch.tv/membership\r\n".to_string();
    // let irc_options = "CAP REQ :twitch.tv/tags\r\n".to_string();

    for channel in &config.channels {
        let join_message = format!("JOIN #{}\r\n", channel);
        tx_msg_push.send(join_message).unwrap();
    }
}
