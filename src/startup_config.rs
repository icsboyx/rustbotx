use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: String,
    pub port: i32,
    pub ssl_tls: bool,
    pub token: String,
    pub nickname: String,
    pub channels: Vec<String>,

}

pub fn load_config() -> Config {
    let config_file = "config.json";
    // Read the contents of the config.json file into a string
    let mut file = File::open(config_file).expect("Failed to open config.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read config.json");

    // Deserialize the JSON content into the Config struct
    let config: Config = serde_json::from_str(&contents).expect("Failed to parse config.json");

    // Print configuration data
    // println!("{:#?}", config);
    // println!("Token: {}", config.token);
    // println!("Nickname: {}", config.nickname);
    // println!("Channels: {:?}", config.channels);
    config
}
