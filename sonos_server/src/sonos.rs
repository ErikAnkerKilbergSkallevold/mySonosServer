use std::error::Error;
use std::time::Duration;
use rayon::prelude::*;
use rusty_sonos::discovery::discover_devices;
use rusty_sonos::speaker::BasicSpeakerInfo;
use tokio::io::AsyncWriteExt;
use tokio::fs::{File};
use crate::utils::serialize_speaker_info;


pub async fn return_devices(search_timeout_ms: u64, read_timeout_ms: u64) -> Result<Vec<BasicSpeakerInfo>, Box<dyn Error>> {
    let search_timeout = Duration::from_millis(search_timeout_ms);
    let read_timeout = Duration::from_millis(read_timeout_ms);

    match discover_devices(search_timeout, read_timeout).await {
        Ok(speaker_info) => {
            // Serialize each BasicSpeakerInfo to a JSON string
            let json_data: String = speaker_info.par_iter().map(serialize_speaker_info).collect::<Vec<String>>().join(",");

            // Write the JSON data to a file asynchronously
            let mut file = File::create("./cache/speaker_info.json").await?;
            file.write_all(json_data.as_bytes()).await?;

            Ok(speaker_info)
        },
        Err(e) => {
            Err(Box::try_from(e).unwrap())
        }
    }
}
