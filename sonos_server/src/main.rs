mod sonos;
mod routes;
mod error_routes;
mod utils;

#[macro_use] extern crate rocket;

use std::error::Error;
use std::time::Duration;
use std::process::Command;
use std::sync::{Arc};
use async_std::task;
use rocket::fs::FileServer;
use rusty_sonos::speaker::BasicSpeakerInfo;
use tokio::sync::Mutex;
use tokio::time::interval;
use crate::error_routes::{internal_error, not_found};
use crate::routes::{get_devices, get_sound_uri, index, play_sound};
use crate::sonos::return_devices;

async fn handle_request(zone: &str, song: &str) -> Result<String, Box<dyn Error>> {
    // Specify the path to the Python script relative to the Rust project's root directory
    let output = Command::new("py")
        .arg("main.py") // Adjust the path as necessary
        .arg(zone) // First parameter: room name
        .arg("--filename") // Second parameter: filename flag
        .arg(song) // Third parameter: filename value
        .output()
        .expect("Failed to execute Python script");

    dbg!(&output);
    task::sleep(Duration::from_millis(1000)).await;
    let response_body = format!("Python script output: {}", String::from_utf8_lossy(&output.stdout));

    Ok(response_body)
}

#[launch]
async fn rocket() -> _ {
    // Initialize the speaker list on launch
    let speaker_list = Arc::new(Mutex::new(Vec::new()));

    // Clone the Arc before locking the mutex
    let speaker_list_clone = Arc::clone(&speaker_list);

    // Now, lock the mutex and update the speaker list
    let mut speaker_list = speaker_list.lock().await;
    *speaker_list = return_devices(1000, 200).await.unwrap_or_else(|_| Vec::new());

    // Spawn a background task to periodically check for online speakers
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(600)); // Check every 10 minutes
        loop {
            interval.tick().await;
            let speakers = return_devices(30000, 2000).await.unwrap_or_else(|_| Vec::new());
            let mut speaker_list = speaker_list_clone.lock().await;
            *speaker_list = speakers;
        }
    });

    rocket::build()
        .mount("/", routes![index])
        .mount("/api", routes![play_sound, get_devices, get_sound_uri])
        .mount("/public", FileServer::from("./static/"))
        .register("/", catchers![not_found, internal_error])
}


