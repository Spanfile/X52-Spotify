use spotify::{track::Track, Spotify};
use std::time::Duration;

fn main() -> Result<(), Box<std::error::Error>> {
    let mut spotify = Spotify::new(Duration::from_secs(1), track_changed);
    Ok(spotify.run()?)
}

fn track_changed(track: Option<Track>) {
    println!("{:?}", track);
}
