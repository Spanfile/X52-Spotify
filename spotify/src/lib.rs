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

#[derive(Debug)]
pub enum Update {
    Tick,
    Status(SpotifyStatus),
}

#[derive(Debug)]
pub enum SpotifyStatus {
    Playing(Track),
    NotPlaying,
    NotRunning,
}

pub struct Spotify<F: FnMut(Update) -> ()> {
    callback: F,
    refresh_interval: Duration,
    previous_title: Option<String>,
}

impl<F> Spotify<F>
where
    F: FnMut(Update) -> () + Send,
{
    pub fn new(refresh_interval: Duration, callback: F) -> Spotify<F> {
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
            // run ticker in separate thread
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
                                            (self.callback)(Update::Status(
                                                SpotifyStatus::Playing(track),
                                            ));
                                        } else {
                                            (self.callback)(Update::Status(
                                                SpotifyStatus::NotPlaying,
                                            ));
                                        }

                                        self.previous_title = Some(title);
                                    }
                                    None => {
                                        (self.callback)(Update::Status(SpotifyStatus::NotRunning));
                                        self.previous_title = None;
                                    }
                                }
                            } else {
                                (self.callback)(Update::Tick);
                            }
                        }
                        Err(e) => println!("error: {:?}", e),
                    }
                }
            });

            // listen for SIGINTs in own thread
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