use spotify::{Spotify, SpotifyStatus};
use std::time::Duration;
use x52::X52;

fn main() -> Result<(), Box<std::error::Error>> {
    let x52_device = X52::new();
    let mut spotify = Spotify::new(Duration::from_secs(1), |status| {
        println!("{:?}", status);
        match status {
            SpotifyStatus::NotRunning => {
                match x52_device.set_lines([
                    String::from("Not running"),
                    String::from(""),
                    String::from(""),
                ]) {
                    Err(e) => println!("failed setting lines: {:?}", e),
                    _ => (),
                };
            }
            SpotifyStatus::NotPlaying => {
                match x52_device.set_lines([
                    String::from("Not playing"),
                    String::from(""),
                    String::from(""),
                ]) {
                    Err(e) => println!("failed setting lines: {:?}", e),
                    _ => (),
                };
            }
            SpotifyStatus::Playing(track) => {
                match x52_device.set_lines([String::from("Now playing:"), track.artist, track.name])
                {
                    Err(e) => println!("failed setting lines: {:?}", e),
                    _ => (),
                };
            }
        };
    });
    Ok(spotify.run()?)
}
