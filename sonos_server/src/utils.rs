use std::net::{Ipv4Addr, UdpSocket};
use rocket::http::Status;
use rusty_sonos::speaker::{BasicSpeakerInfo, Speaker};
use serde_json::json;
use regex::Regex;

pub async fn get_local_ip() -> std::io::Result<String> {
    // Create a UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // Connect to a remote address
    let remote_addr = "8.8.8.8:80";
    socket.connect(remote_addr)?;

    // Get the local socket address
    let local_addr = socket.local_addr()?;

    // Extract the IP address as a string
    let ip_address = local_addr.ip().to_string();

    Ok(ip_address)
}

pub fn serialize_speaker_info(speaker_info: &BasicSpeakerInfo) -> String {
    let re = Regex::new(r"- (.+?) -").unwrap();
    let hay = speaker_info.friendly_name();
    let caps = re.captures(hay).unwrap();
    let captured_name = &caps[1];

    let json_object = json!({
        "name": speaker_info.friendly_name(),
        "room": captured_name,
        "ip_address": speaker_info.ip_addr(),
        "uuid": speaker_info.uuid(),
    });
    // Convert the JSON object to a string
    serde_json::to_string(&json_object).unwrap()
}

pub async fn create_sound_uri(sound: &str) -> Result<String, Status> {
    let config = rocket::config::Config::figment().extract::<rocket::Config>().unwrap();
    
    let ip_addr = config.address;
    let port = config.port;

    Ok(format!("http://{}:{}/public/{}", ip_addr, port, sound))
}
