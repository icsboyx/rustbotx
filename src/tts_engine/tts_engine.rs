use tts_rust::languages::Languages;
use tts_rust::tts::GTTSClient;

pub fn tts_engine(payload: String) {
    let narrator = GTTSClient {
        volume: 1.0,
        language: Languages::English,
        tld: "it", // use the Languages enum
    };
    println!("{}", payload);
    narrator.display_and_speak(&payload[..30]);
}
