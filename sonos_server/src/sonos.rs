use crate::utils::{create_sound_uri, format_speaker_name, serialize_speaker_info};
use rayon::prelude::*;
use rocket::http::Status;
use rusty_sonos::discovery::discover_devices;
use rusty_sonos::speaker::{BasicSpeakerInfo, Speaker};
use std::error::Error;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn return_devices(
    search_timeout_ms: u64,
    read_timeout_ms: u64,
) -> Result<Vec<BasicSpeakerInfo>, Box<dyn Error>> {
    let search_timeout = Duration::from_millis(search_timeout_ms);
    let read_timeout = Duration::from_millis(read_timeout_ms);

    match discover_devices(search_timeout, read_timeout).await {
        Ok(speaker_info) => {
            // Serialize each BasicSpeakerInfo to a JSON string
            let json_data: String = speaker_info
                .par_iter()
                .map(serialize_speaker_info)
                .collect::<Vec<String>>()
                .join(",");

            // Write the JSON data to a file asynchronously
            let mut file = File::create("./cache/speaker_info.json").await?;
            file.write_all(json_data.as_bytes()).await?;

            Ok(speaker_info)
        }
        Err(e) => Err(Box::from(e)),
    }
}

pub async fn play_sound_on_sonos(
    speaker_room: &str,
    speaker_name: &str,
    volume: u8,
    sound: &str,
) -> Result<(), Status> {
    let speakers = match return_devices(700, 200).await {
        Ok(data) => data,
        Err(_) => return Err(Status::InternalServerError),
    };
    dbg!(&speakers);

    let speaker_info = match speakers
        .iter()
        .find(|speaker| {
            format_speaker_name(speaker.friendly_name()).unwrap() == speaker_name
                && speaker.room_name() == speaker_room
        })
        .ok_or(Status::InternalServerError)
    {
        Ok(speaker) => speaker,
        Err(_) => return Err(Status::InternalServerError),
    };
    dbg!(&speaker_info);

    let ip_addr = speaker_info.ip_addr();
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

    // Pause the speaker
    /*
    match speaker.pause().await {
        Ok(_) => (),
        Err(e) => {
            dbg!(e);
            return Err(Status::InternalServerError)
        },
    }
    dbg!("paused");
     */

    // Set Volume 0-100
    match speaker.set_volume(volume).await {
        Ok(_) => (),
        Err(_) => return Err(Status::InternalServerError),
    }
    dbg!("volume");

    // Set the song URI
    match speaker.set_current_uri(sound_uri.as_str()).await {
        Ok(_) => (),
        Err(_) => return Err(Status::InternalServerError),
    };
    dbg!("sound_uri");

    // Play
    match speaker.play().await {
        Ok(_) => (),
        Err(_) => return Err(Status::InternalServerError),
    };
    dbg!("play");

    Ok(())
}
