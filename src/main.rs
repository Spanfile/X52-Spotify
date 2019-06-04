use spotify::Spotify;

fn main() {
    let spotify = Spotify::new();
    println!("{:?}", spotify.get_track());
}
