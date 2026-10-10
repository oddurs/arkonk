//! Loading and playing sounds.

#![allow(warnings)]

mod error;

pub use error::Error;

#[cfg(target_os = "android")]
#[path = "opensles_snd.rs"]
mod snd;

#[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"))]
#[path = "alsa_snd.rs"]
mod snd;

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[path = "coreaudio_snd.rs"]
mod snd;

#[cfg(target_os = "windows")]
#[path = "wasapi_snd.rs"]
mod snd;

#[cfg(target_arch = "wasm32")]
#[path = "web_snd.rs"]
mod snd;

#[cfg(not(target_arch = "wasm32"))]
mod mixer;

pub use snd::{AudioContext, Playback, Sound};

#[cfg(not(target_arch = "wasm32"))]
pub use output::start_output;

/// ARKONK: the desktop backends park their device start here instead of
/// running it in `AudioContext::new`, which macroquad calls on the main thread
/// while it builds its context. The game decides whether and when to start the
/// device, and nothing on the main thread waits for it.
#[cfg(not(target_arch = "wasm32"))]
mod output {
    use std::sync::{mpsc, Mutex};

    /// Runs on its own thread. It reports `Ok` once the device is playing, or
    /// why it could not start; dropping the sender unsent (a panic) also
    /// means it failed.
    pub(crate) type Start = Box<dyn FnOnce(mpsc::Sender<Result<(), String>>) + Send>;

    static PENDING: Mutex<Option<Start>> = Mutex::new(None);

    pub(crate) fn park(start: Start) {
        if let Ok(mut pending) = PENDING.lock() {
            *pending = Some(start);
        }
    }

    /// Starts the parked output device on a new thread and returns where its
    /// outcome arrives. Until it succeeds, sounds load and decode as usual but
    /// nothing reaches the device.
    pub fn start_output() -> mpsc::Receiver<Result<(), String>> {
        let (ready, outcome) = mpsc::channel();
        let start = PENDING.lock().ok().and_then(|mut pending| pending.take());
        match start {
            Some(start) => {
                let ready_on_failure = ready.clone();
                let spawned = std::thread::Builder::new()
                    .name("audio".into())
                    .spawn(move || start(ready));
                if let Err(e) = spawned {
                    let why = format!("could not start the audio thread: {e}");
                    let _ = ready_on_failure.send(Err(why));
                }
            }
            None => {
                let _ = ready.send(Err("no audio output is waiting to start".into()));
            }
        }
        outcome
    }
}

pub struct PlaySoundParams {
    pub looped: bool,
    pub volume: f32,
}

impl Default for PlaySoundParams {
    fn default() -> PlaySoundParams {
        PlaySoundParams {
            looped: false,
            volume: 1.,
        }
    }
}
