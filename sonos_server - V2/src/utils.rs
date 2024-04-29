use regex::Regex;
use rocket::http::Status;
use std::error::Error;

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
