use spotify::{Spotify, SpotifyStatus, Update};
use std::time::Duration;
use x52::X52;

fn main() -> Result<(), Box<std::error::Error>> {
    let mut x52_device = X52::new();
    let mut spotify = Spotify::new(Duration::from_secs(1), |update| {
        println!("{:?}", update);
        match update {
            Update::Status(status) => {
                match status {
                    SpotifyStatus::NotRunning => {
                        if let Err(e) = x52_device.set_lines([
                            String::from("Not running"),
                            String::from(""),
                            String::from(""),
                        ]) {
                            println!("failed setting lines: {:?}", e);
                        };
                    }
                    SpotifyStatus::NotPlaying => {
                        if let Err(e) = x52_device.set_lines([
                            String::from("Not playing"),
                            String::from(""),
                            String::from(""),
                        ]) {
                            println!("failed setting lines: {:?}", e);
                        };
                    }
                    SpotifyStatus::Playing(track) => {
                        if let Err(e) = x52_device.set_lines([
                            String::from("Now playing:"),
                            track.artist,
                            track.name,
                        ]) {
                            println!("failed setting lines: {:?}", e);
                        };
                    }
                };
                if let Err(e) = x52_device.tick() {
                    println!("failed X52 tick: {:?}", e);
                }
            }
            Update::Tick => {
                if let Err(e) = x52_device.tick() {
                    println!("failed X52 tick: {:?}", e);
                }
            }
        };
    });
    Ok(spotify.run()?)
}
