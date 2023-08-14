use ctrlc::set_handler;
use std::thread;
use std::time::Duration;
use colored::*;



// Import the external custom modules
mod standard_console;
mod startup_config;
pub mod irc_engine;
use crate::irc_engine::irc_connect;

fn main() {
    let config = startup_config::load_config();
    // let console = console::Console::new(16);

    irc_connect(config);

    c_println!("Main Thread ###################################");


   // Register the CTRL+C signal handler
    set_handler(move || {
        // println!("CTRL+C signal received. Terminating...");
        println!("{}","CTRL+C signal received. Terminating...".red().bold().underline());
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    loop {
        // Sleep for 1 second to avoid busy waiting and reduce CPU usage
        thread::sleep(Duration::from_secs(1));
    }
}

