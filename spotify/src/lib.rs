pub mod track;
mod window;

use crossbeam::thread;
use signal_hook::{iterator::Signals, SIGINT};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;
use track::Track;
use window::SpotifyWindow;

pub type TrackCallback = fn(status: SpotifyStatus);

pub struct Spotify {
    callback: TrackCallback,
    refresh_interval: Duration,
    previous_title: Option<String>,
}

#[derive(Debug)]
pub enum SpotifyStatus {
    Playing(Track),
    NotPlaying,
    NotRunning,
}

impl Spotify {
    pub fn new(refresh_interval: Duration, callback: TrackCallback) -> Spotify {
        Spotify {
            callback,
            refresh_interval,
            previous_title: None,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<std::error::Error>> {
        let signals = Signals::new(&[SIGINT])?;
        let interrupt_flag = Arc::new(AtomicBool::new(false));

        thread::scope(|s| {
            let interrupt_flag_clone = Arc::clone(&interrupt_flag);
            s.spawn(move |_| {
                let window = SpotifyWindow::new();
                loop {
                    if interrupt_flag_clone.load(Ordering::Relaxed) {
                        println!("breaking");
                        break;
                    }

                    std::thread::sleep(self.refresh_interval);
                    // println!("tick");

                    match window.get_title() {
                        Ok(title) => {
                            if title != self.previous_title {
                                match title {
                                    Some(title) => {
                                        if let Some(track) = Track::build(&title) {
                                            (self.callback)(SpotifyStatus::Playing(track));
                                        } else {
                                            (self.callback)(SpotifyStatus::NotPlaying);
                                        }

                                        self.previous_title = Some(title);
                                    }
                                    None => {
                                        (self.callback)(SpotifyStatus::NotRunning);
                                        self.previous_title = None;
                                    }
                                }

                            }
                        }
                        Err(e) => println!("error: {:?}", e),
                    }
                }
            });

            for sig in signals.forever() {
                match sig {
                    SIGINT => {
                        println!("interrupted");
                        interrupt_flag.store(true, Ordering::Relaxed);
                        break;
                    }
                    _ => unreachable!(),
                }
            }
        })
        .expect("failed joining ticker thread");

        Ok(())
    }
}