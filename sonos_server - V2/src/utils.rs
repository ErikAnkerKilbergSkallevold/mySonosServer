use regex::Regex;
use rocket::http::Status;
use rusty_sonos::speaker::BasicSpeakerInfo;
use serde_json::json;
use std::error::Error;

pub fn serialize_speaker_info(speaker_info: &BasicSpeakerInfo) -> String {
    /*
    let re = Regex::new(r"- (.+?) -").unwrap();
    let hay = speaker_info.friendly_name();
    let caps = re.captures(hay).unwrap();
    let captured_name = &caps[1];
    */
    let json_object = json!({
        "name": speaker_info.friendly_name(),
        "room": speaker_info.room_name(),
        "ip_address": speaker_info.ip_addr(),
        "uuid": speaker_info.uuid(),
    });
    // Convert the JSON object to a string
    serde_json::to_string(&json_object).unwrap()
}

pub fn format_speaker_name(speaker_name: &str) -> Result<String, Status> {
    let re = Regex::new(r"- (.+?) -").unwrap();
    let hay = speaker_name;
    let caps = re.captures(hay).unwrap();
    let captured_name = &caps[1];
    Ok(captured_name.to_string())
}

pub async fn create_sound_uri(sound: &str) -> Result<String, Box<dyn Error>> {
    let config = rocket::config::Config::figment().extract::<rocket::Config>()?;

    let ip_addr = config.address;
    let port = config.port;

    Ok(format!("http://{}:{}/public/{}", ip_addr, port, sound))
}
