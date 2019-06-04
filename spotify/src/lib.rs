pub struct Spotify {}

#[derive(Debug)]
pub struct Track {
    pub name: String,
    pub artist: String,
}

impl Spotify {
    pub fn new() -> Spotify {
        Spotify {}
    }

    pub fn get_track(&self) -> Track {
        Track {
            name: String::from(""),
            artist: String::from(""),
        }
    }
}