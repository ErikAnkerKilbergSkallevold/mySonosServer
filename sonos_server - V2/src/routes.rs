use std::time::Duration;
use crate::sonos::Speakercontrols::{Pause, Play};
use crate::sonos::{
    apply_snapshot, control_speaker, find_speaker, set_song, set_volume, take_snapshot,
};
use crate::utils::create_sound_uri;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use tokio::time::sleep;
use crate::lofty::get_sound_duration_ms;

#[derive(Deserialize)]
pub struct RequestBody<'a> {
    sound: &'a str,
    volume: u16,
    roomnames: Vec<String>,
}

#[get("/")]
pub async fn index() -> &'static str {
    "Hello, World!"
}

#[post("/play", format = "json", data = "<request_body>")]
pub async fn play_sound_in_rooms(request_body: Json<RequestBody<'_>>) -> String {
    // Your existing logic here, adjusted to iterate over room_names.roomnames
    // For example, to play sound in each room:
    let mut result = String::new();
    let volume = request_body.volume;
    let sound = request_body.sound;

    for roomname in request_body.0.roomnames {
        // Find all speakers with roomname
        let speaker = match find_speaker(&roomname, 1000).await {
            Ok(speaker) => match speaker {
                Some(speaker) => speaker,
                _ => return "No speaker found".to_string(),
            },
            Err(e) => return e.to_string(),
        };

        let sound_uri = match create_sound_uri(sound).await {
            Ok(uri) => uri,
            Err(e) => return e.to_string(),
        };

        // Take a snapshot of the speaker
        let snapshot = match take_snapshot(&speaker).await {
            Ok(snapshot) => snapshot,
            Err(e) => return e.to_string(),
        };

        // Pause
        match control_speaker(&speaker, Pause).await {
            Ok(_) => (),
            Err(e) => return e.to_string(),
        };

        // Set Volume
        let _ = match set_volume(&speaker, volume).await {
            Ok(vol) => vol,
            Err(e) => return e.to_string(),
        };

        // Set sound
        match set_song(&speaker, &sound_uri).await {
            Ok(_) => (),
            Err(e) => return e.to_string(),
        };

        // Play
        match control_speaker(&speaker, Play).await {
            Ok(_) => (),
            Err(e) => return e.to_string(),
        };

        // Sleep until the song is over 
        let duration = match get_sound_duration_ms(sound).await {
            Ok(duration) => duration,
            Err(e) => return e.to_string(),
        };
        sleep(Duration::from_millis(duration as u64)).await;
        
        // Restore snapshot
        match apply_snapshot(&speaker, snapshot).await {
            Ok(_) => (),
            Err(e) => return e.to_string(),
        };

        result.push_str(&format!(
            "Playing {} with uri: {} on speaker {:?} in room: {} at volume: {}\n",
            sound, &sound_uri, speaker, roomname, volume
        ));
    }
    result
}

#[get("/sound/<sound>")]
pub async fn get_sound_uri(sound: &str) -> Result<String, Status> {
    match create_sound_uri(sound).await {
        Ok(uri) => Ok(uri.parse().unwrap()),
        Err(_) => Err(Status::NotFound),
    }
}
