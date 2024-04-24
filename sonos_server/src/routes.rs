use crate::sonos::{play_sound_on_sonos, return_devices};
use crate::utils::create_sound_uri;
use rayon::prelude::*;
use regex::Regex;
use rocket::http::Status;

#[get("/")]
pub async fn index() -> &'static str {
    "Hello, World!"
}

#[get("/play_sound/<speaker_room>/<speaker_name>/<volume>/<sound>")]
pub async fn play_sound(
    speaker_room: &str,
    speaker_name: &str,
    volume: u8,
    sound: &str,
) -> Result<String, Status> {
    match play_sound_on_sonos(speaker_room, speaker_name, volume, sound).await {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

    Ok(format!(
        "Playing sound: {} on speaker: {} in room: {}",
        sound, speaker_name, speaker_room
    ))
}

#[get("/sound/<sound>")]
pub async fn get_sound_uri(sound: &str) -> Result<String, Status> {
    match create_sound_uri(sound).await {
        Ok(uri) => Ok(uri),
        Err(_) => Err(Status::NotFound),
    }
}

#[get("/devices")]
pub async fn get_devices() -> Result<String, Status> {
    match return_devices(5000, 1000).await {
        Ok(speaker_info_vec) => {
            // Iterate over each BasicSpeakerInfo and format it into a string.
            let speaker_info_strings: Vec<String> = speaker_info_vec
                .into_par_iter()
                .map(|speaker_info| {
                    let re = Regex::new(r"- (.+?) -").unwrap();
                    let hay = speaker_info.friendly_name();
                    let caps = re.captures(hay).unwrap();
                    let captured_name = &caps[1];
                    format!(
                        "IP: {}, Friendly Name: {}, Room Name: {}, UUID: {}",
                        speaker_info.ip_addr(),
                        captured_name,
                        speaker_info.room_name(),
                        speaker_info.uuid()
                    )
                })
                .collect();

            // Join all strings into a single string, separated by a newline.
            let all_speaker_info = speaker_info_strings.join("\n");

            Ok(all_speaker_info)
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
