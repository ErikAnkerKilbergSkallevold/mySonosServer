use async_std::path::Path;
use lofty::prelude::AudioFile;
use lofty::{read_from_path};
use std::error::Error;

pub async fn get_sound_duration_ms(filename: &str) -> Result<u128, Box<dyn Error>> {
    let path = Path::new("./static/").join(filename);
    let tagged_file = read_from_path(&path)?;

    let duration = tagged_file.properties().duration().as_millis();

    Ok(duration)
}
