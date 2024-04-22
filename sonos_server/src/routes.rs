
use std::net::Ipv4Addr;
use std::str::FromStr;


use rocket::http::Status;

use crate::sonos::return_devices;
use rayon::prelude::*;
use rusty_sonos::speaker::{Speaker};
use serde_json::Value;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::utils::{create_sound_uri, serialize_speaker_info};

#[get("/")]
pub async fn index() -> &'static str {
    "Hello, World!"
}

#[get("/play_sound/<speaker_room>/<speaker_name>/<sound>")]
pub async fn play_sound(speaker_room: &str, speaker_name: &str, sound: &str) -> Result<String, Status> {

    // Attempt to read the cache file
    let mut cache_data = match fs::read_to_string("./cache/speaker_info.json").await {
        Ok(data) => data,
        Err(_) => return Err(Status::InternalServerError),
    };
    dbg!(&cache_data);

    if cache_data.is_empty() {
        dbg!("Cache data empty! Attempting to retrieve devices again.");
        match return_devices(1000, 200).await {
            Ok(data) => {
                // Serialize each BasicSpeakerInfo to a JSON string
                cache_data = data.par_iter().map(serialize_speaker_info).collect::<Vec<String>>().join(",");
            },
            Err(_) => return Err(Status::InternalServerError),
        };
    }
    dbg!(&cache_data);

    // Deserialize the cache data into a JSON Value for simplicity
    let cache_json: Value = match serde_json::from_str(&cache_data) {
        Ok(json) => json,
        Err(_) => return Err(Status::InternalServerError),
    };
    dbg!(&cache_json);

    // Search for the speaker by name in the cache
    let speaker_info = cache_json.as_array()
        .and_then(|arr| arr.iter().find(|v| v["name"].as_str() == Some(speaker_name) && v["room"].as_str() == Some(speaker_room)))
        .ok_or(Status::InternalServerError)?;
    dbg!(&speaker_info);

    // Extract the IP address from the speaker info
    let ip_str = speaker_info["ip_address"].as_str().ok_or(Status::InternalServerError)?;
    dbg!(&ip_str);
    let ip_addr = match Ipv4Addr::from_str(ip_str) {
        Ok(addr) => addr,
        Err(_) => return Err(Status::InternalServerError),
    };
    dbg!(&ip_addr);

    // Create the Speaker object
    let speaker = match Speaker::new(ip_addr).await {
        Ok(speaker) => speaker,
        Err(_) => return Err(Status::InternalServerError),
    };

    // Get sound URI
    let sound_uri = match create_sound_uri(sound).await {
        Ok(uri) => uri,
        Err(_) => return Err(Status::NotFound),
    };
    dbg!(&sound_uri);

    // Attempt to play the song
    match speaker.set_current_uri(sound_uri.as_str()).await {
        Ok(_) => (),
        Err(_) => return Err(Status::InternalServerError),
    };

    Ok(String::from(format!("Playing sound: {} on speaker: {} in room: {}", sound, speaker.get_friendly_name(), speaker_room)))
}

#[get("/sound/<sound>")]
pub async fn get_sound_uri(sound: &str) -> Result<String, Status> {
    return match create_sound_uri(sound).await {
        Ok(uri) => Ok(uri),
        Err(_) => return Err(Status::NotFound),
    };
}

#[get("/devices")]
pub async fn get_devices() -> Result<String, Status> {
    match return_devices(30000, 2000).await {
        Ok(speaker_info_vec) => {
            // Iterate over each BasicSpeakerInfo and format it into a string.
            let speaker_info_strings: Vec<String> = speaker_info_vec.into_par_iter()
                .map(|speaker_info| {
                    format!(
                        "IP: {}, Friendly Name: {}, Room Name: {}, UUID: {}",
                        speaker_info.ip_addr(),
                        speaker_info.friendly_name(),
                        speaker_info.room_name(),
                        speaker_info.uuid()
                    )
                })
                .collect();

            // Join all strings into a single string, separated by a newline.
            let all_speaker_info = speaker_info_strings.join("\n");

            Ok(all_speaker_info)
        },
        Err(_) => Err(Status::InternalServerError),
    }
}


