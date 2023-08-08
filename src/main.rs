use ctrlc::set_handler;
use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use colored::*;

// Import the external custom modules
mod standard_console;
// use standard_console::LocalConsole;
mod receive_messages;
mod send_messages;
mod startup_config;
pub mod irc_engine;

fn main() {
    let config = startup_config::load_config();
    // let console = console::Console::new(16);


    c_println!("Main Thread ###################################");


   // Register the CTRL+C signal handler
    set_handler(move || {
        // println!("CTRL+C signal received. Terminating...");
        println!("{}","CTRL+C signal received. Terminating...".red().bold().underline());
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");


 
    match TcpStream::connect("irc.chat.twitch.tv:6667") {
        Ok(mut stream) => {
            // println!("Connected to Twitch IRC server");
            c_println!("Connected to Twitch IRC server");

            // Clone the stream for the thread that will receive data from socket.
            let stream_clone_receive = stream.try_clone().expect(
                "Failed to clone  the stream for the thread that will receive data from socket",
            );
            // Start a new thread to handle message receiving
            thread::spawn(move || {
                receive_messages::receive_messages(stream_clone_receive);
            });

            // // Clone the stream for the thread that read from user input.
            // let stream_clone_user_input = stream
            //     .try_clone()
            //     .expect("Failed to clone the stream for the thread that will read data user input");
            // // Start a new thr#[macro_export]
            // Send authentication message to the IRC server
            let pass_message = format!("PASS oauth:{}\r\n", config.token);
            let nick_message = format!("NICK {}\r\n", config.nickname);
            stream
                .write_all(pass_message.as_bytes())
                .expect("Failed to write to stream");
            stream
                .write_all(nick_message.as_bytes())
                .expect("Failed to write to stream");

                c_println!("Authentication message sent");

            // Join the specified channels
            for channel in &config.channels {
                let join_message = format!("JOIN #{}\r\n", channel);
                stream
                    .write_all(join_message.as_bytes())
                    .expect("Failed to write to stream");
                c_println!(format!("Joining channel: {}", channel));
                // let _message = format!("Hello from the bot!");
                //send_messages(&stream, channel, message);
            }
        }

        // Err(e) => eprintln!("Error connecting to Twitch IRC server: {}", e),
        Err(e) => println!("Error connecting to Twitch IRC server: {}", e),
    }
    loop {
        // Sleep for 1 second to avoid busy waiting and reduce CPU usage
        thread::sleep(Duration::from_secs(1));
    }
}

