#[derive(Debug, PartialEq)]
pub struct Track {
    pub name: String,
    pub artist: String,
}

impl Track {
    pub fn build(s: &str) -> Option<Track> {
        let splits: Vec<&str> = s.splitn(2, " - ").collect();
        if splits.len() == 2 {
            Some(Track {
                artist: String::from(splits[0]),
                name: String::from(splits[1]),
            })
        } else {
            None
        }
    }
}