use sonor::{Snapshot, Speaker};
use std::error::Error;
use std::time::Duration;

pub enum Speakercontrols {
    Play,
    Pause,
    Stop,
    Next,
    Previous,
    Mute,
    Unmute,
}

pub async fn find_speaker(
    roomname: &str,
    timeout_ms: u64,
) -> Result<Option<Speaker>, Box<dyn Error>> {
    return match sonor::find(roomname, Duration::from_millis(timeout_ms)).await {
        Ok(speaker) => Ok(speaker),
        Err(e) => Err(Box::from(e)),
    };
}

pub async fn set_volume(speaker: &Speaker, volume: u16) -> Result<u16, Box<dyn Error>> {
    speaker.set_volume(volume).await?;
    Ok(speaker.volume().await?)
}

pub async fn control_speaker(
    speaker: &Speaker,
    action: Speakercontrols,
) -> Result<(), Box<dyn Error>> {
    match action {
        Speakercontrols::Play => Ok(speaker.play().await?),
        Speakercontrols::Pause => Ok(speaker.pause().await?),
        Speakercontrols::Next => Ok(speaker.next().await?),
        Speakercontrols::Previous => Ok(speaker.previous().await?),
        Speakercontrols::Stop => Ok(speaker.stop().await?),
        Speakercontrols::Mute => Ok(speaker.set_mute(true).await?),
        Speakercontrols::Unmute => Ok(speaker.set_mute(false).await?),
    }
}

pub async fn take_snapshot(speaker: &Speaker) -> Result<Snapshot, Box<dyn Error>> {
    Ok(speaker.snapshot().await?)
}

pub async fn apply_snapshot(speaker: &Speaker, snapshot: Snapshot) -> Result<(), Box<dyn Error>> {
    let _ = speaker.apply(snapshot).await?;
    Ok(())
}

pub async fn set_song(speaker: &Speaker, uri: &str) -> Result<(), Box<dyn Error>> {
    let _ = speaker.set_transport_uri(uri, "").await?;
    Ok(())
}

pub async fn join(speaker: &Speaker, roomname: &str) -> Result<bool, Box<dyn Error>> {
    Ok(speaker.join(roomname).await?)
}

pub async fn leave(speaker: &Speaker) -> Result<(), Box<dyn Error>> {
    Ok(speaker.leave().await?)
}
