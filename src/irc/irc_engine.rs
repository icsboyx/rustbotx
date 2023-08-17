use chrono::Local;
use colored::*;
use ctrlc::set_handler;
use openssl::ssl::{SslConnector, SslMethod, SslStream};
use openssl_sys::{SS, RTEXT_FILTER_CFM_STATUS};
use std::io::{Read, Write, self};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{process, thread};

use crate::startup_config::Config;

#[derive(Debug)]
enum StreamType {
    MyTcpStream(Arc<Mutex<TcpStream>>,Arc<Mutex<TcpStream>>),
    MySslStream(Arc<Mutex<SslStream<TcpStream>>>,Arc<Mutex<SslStream<TcpStream>>>),
}


pub fn irc_connect(config: Config) {
    


    // Connect to localhost:667 using a TcpStream.
    let tcp_stream = create_tcp(&config).unwrap();

    
    let create_streams = if !config.ssl_tls {
       create_tcp_streams(tcp_stream)
    }else {
        create_ssl_streams(tcp_stream,config)
    };

    let rx_stream: Arc<Mutex<dyn Read + Send>>; // Define the receive stream
    let tx_stream: Arc<Mutex<dyn Write + Send>>; // Define the transmit stream


    match create_streams {
        StreamType::MyTcpStream(rx, tx) => {
            rx_stream = rx as Arc<Mutex<dyn Read + Send>>; // Convert to the appropriate trait object
            tx_stream = tx as Arc<Mutex<dyn Write + Send>>; // Convert to the appropriate trait object
        }
        StreamType::MySslStream(rx, tx) => {
            rx_stream = rx as Arc<Mutex<dyn Read + Send>>; // Convert to the appropriate trait object
            tx_stream = tx as Arc<Mutex<dyn Write + Send>>; // Convert to the appropriate trait object
        }
    }

    // Perform read operations on the SSL stream
    let _ = thread::Builder::new().name("receiver_thread".to_string()).spawn(move || loop {
        let mut rx_payload = String::new();
        match rx_stream
            .lock()
            .unwrap()
            .read_to_string(&mut rx_payload)
        {
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

            println!("{}", rx_payload);
        }
        thread::sleep(Duration::from_millis(100));
    });

    // Perform write operations on the SSL stream
    let _ = thread::Builder::new().name("sender_thread".to_string()).spawn(move || loop {
        let tx_payload = Local::now();
        let tx_payload = tx_payload.format("%Y-%m-%d %H:%M:%S\r\n").to_string();
        tx_stream
            .lock()
            .unwrap()
            .write_all(tx_payload.as_bytes())
            .unwrap();
        thread::sleep(Duration::from_secs(1));
    });


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



// fn irc_authentication(config: Config) {
//     let pass_message = format!("PASS oauth:{}\r\n", config.token);
//     let nick_message = format!("NICK {}\r\n", config.nickname);

//     let irc_options = "CAP REQ :twitch.tv/commands\r\n".to_string();
//     // let irc_options = "CAP REQ :twitch.tv/membership\r\n".to_string();
//     // let irc_options = "CAP REQ :twitch.tv/tags\r\n".to_string();

//     // for channel in config.channels {
//     //     let join_message = format!("JOIN #{}\r\n", channel);
//     //     irc_sender.send(join_message).unwrap();
//     // }
// }

fn create_tcp(config: &Config) -> Result<TcpStream, io::Error> {
    // Combine server address and port into a single string
    let address = format!("{}:{}", config.server, config.port);
    // Connect to localhost:667 using a TcpStream.
    TcpStream::connect(address)
}

fn create_tcp_streams(
    tcp_stream: TcpStream,
) ->
StreamType
{
    tcp_stream.set_nonblocking(true).unwrap();
    let tcp_stream = Arc::new(Mutex::new(tcp_stream));
    let rx_stream = tcp_stream.clone();
    let tx_stream = tcp_stream.clone();
    StreamType::MyTcpStream(rx_stream,tx_stream)
}

#[allow(clippy::type_complexity)]
fn create_ssl_streams(
    tcp_stream: TcpStream,
    config: Config,
) -> StreamType{
       // Create an OpenSSL SSL connector.
   let mut connector = SslConnector::builder(SslMethod::tls()).unwrap();
   connector.set_verify(openssl::ssl::SslVerifyMode::NONE);
   let connector = connector.build();
    // Wrap the TcpStream in an SSL stream.
    let ssl_stream = connector.connect(config.server.as_str(), tcp_stream).unwrap();
    // Get a Tcp stream out from the connector.
    let tcp_stream_after_connect = ssl_stream.get_ref();
    // Set a nonblocking for read operations (optional), Must be done here not before connect.
    tcp_stream_after_connect.set_nonblocking(true).unwrap();
    // Create a Arc Mutex for cloning the connector
    let ssl_stream = Arc::new(Mutex::new(ssl_stream));
    // Clone ssl_stream for tx and rx operations.
    let rx_stream = ssl_stream.clone();
    let tx_stream = ssl_stream.clone();
    StreamType::MySslStream(rx_stream,tx_stream)
    }

