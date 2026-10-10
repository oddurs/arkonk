use ark::Events;
use macroquad::audio::{PlaySoundParams, Sound, load_sound_from_bytes, play_sound};
use std::{
    sync::mpsc::{Receiver, TryRecvError},
    time::{Duration, Instant},
};

/// A device that has not answered by then is treated as broken for the
/// session. A healthy one starts in milliseconds; a wedged coreaudiod takes
/// 15 seconds or more and then fails.
const START_TIMEOUT: Duration = Duration::from_secs(3);

/// The OS output device. macroquad builds its audio context with the device
/// start parked (vendor/quad-snd/ARKONK.md); this decides whether to run it,
/// and the frame loop polls the outcome so no frame waits on the device.
enum Output {
    /// A silent run never asks for the device.
    Off,
    Starting {
        outcome: Receiver<Result<(), String>>,
        since: Instant,
    },
    On,
    Failed,
}

impl Output {
    fn begin(silent: bool, start: impl FnOnce() -> Receiver<Result<(), String>>) -> Self {
        if silent {
            return Self::Off;
        }
        Self::Starting {
            outcome: start(),
            since: Instant::now(),
        }
    }

    /// Settles a pending start. Returns the line to log on the one poll where
    /// it settles, and `None` on every other.
    fn poll(&mut self, now: Instant) -> Option<Result<String, String>> {
        let Self::Starting { outcome, since } = self else {
            return None;
        };
        let waited = now.saturating_duration_since(*since);
        let settled = match outcome.try_recv() {
            Ok(Ok(())) => Ok(format!("Audio output started in {} ms", waited.as_millis())),
            Ok(Err(why)) => Err(format!(
                "Audio output failed to start: {why}; playing silently"
            )),
            Err(TryRecvError::Disconnected) => {
                Err("Audio output failed to start: its thread stopped; playing silently".into())
            }
            Err(TryRecvError::Empty) if waited >= START_TIMEOUT => Err(format!(
                "Audio output did not start within {} s; playing silently",
                START_TIMEOUT.as_secs()
            )),
            Err(TryRecvError::Empty) => return None,
        };
        *self = if settled.is_ok() {
            Self::On
        } else {
            Self::Failed
        };
        Some(settled)
    }
}

pub struct Audio {
    clips: [Option<Sound>; 17],
    pub muted: bool,
    pub volume: f32,
    output: Output,
}
impl Audio {
    /// `silent` still decodes every clip, so a test run checks they load, but
    /// it never opens the output device.
    pub async fn new(silent: bool) -> Self {
        let mut clips = std::array::from_fn(|_| None);
        for (i, (start, end, seconds)) in [
            (640.0, 330.0, 0.07),
            (240.0, 450.0, 0.07),
            (170.0, 140.0, 0.035),
            (260.0, 65.0, 0.35),
            (440.0, 1320.0, 0.3),
            (500.0, 1000.0, 0.2),
            (180.0, 700.0, 0.14),
        ]
        .into_iter()
        .enumerate()
        {
            // Synthesize and decode once at startup; gameplay only plays handles.
            let wave = tone(start, end, seconds, i == 4);
            clips[i] = load_sound_from_bytes(&wave).await.ok();
        }
        for (i, slot) in clips.iter_mut().enumerate().take(14).skip(7) {
            let pitch = 2.0_f32.powf((i - 6) as f32 / 12.0);
            *slot = load_sound_from_bytes(&tone(640.0 * pitch, 330.0 * pitch, 0.07, false))
                .await
                .ok();
        }
        for (index, (start, end, duration)) in [
            (360.0, 140.0, 0.11),
            (920.0, 460.0, 0.10),
            (1080.0, 810.0, 0.06),
        ]
        .into_iter()
        .enumerate()
        {
            clips[14 + index] = load_sound_from_bytes(&tone(start, end, duration, false))
                .await
                .ok();
        }
        let audio = Self {
            clips,
            muted: false,
            volume: 0.6,
            output: Output::begin(silent, quad_snd::start_output),
        };
        crate::diagnostics::info(format_args!(
            "Sounds loaded: {}/17, {}",
            audio.loaded(),
            audio.output()
        ));
        audio
    }
    /// Once a frame: logs the device start once it settles.
    pub fn poll(&mut self) {
        match self.output.poll(Instant::now()) {
            Some(Ok(line)) => crate::diagnostics::info(line),
            Some(Err(line)) => crate::diagnostics::error(line),
            None => {}
        }
    }
    pub fn output(&self) -> &'static str {
        match self.output {
            Output::Off => "output not opened (silent run)",
            Output::Starting { .. } => "output starting",
            Output::On => "output on",
            Output::Failed => "output failed",
        }
    }
    pub fn play(&self, e: Events) {
        // Without a mixer thread every play would only print "Audio thread died".
        if !self.audible() || crate::diagnostics::worker_panicked() {
            return;
        }
        for (i, enabled) in [
            e.brick && !e.phase_hit && !e.relay,
            e.paddle && !e.caught,
            e.wall && !e.brick && !e.paddle,
            e.lost,
            e.clear,
            e.pickup,
            e.launch,
            e.relay,
            e.caught,
            e.phase_hit,
        ]
        .into_iter()
        .enumerate()
        {
            let clip = if i == 0 && e.combo > 1 {
                5 + e.combo.min(8) as usize
            } else if i >= 7 {
                i + 7
            } else {
                i
            };
            if enabled && let Some(sound) = &self.clips[clip] {
                play_sound(
                    sound,
                    PlaySoundParams {
                        looped: false,
                        volume: self.volume
                            * if i == 2 {
                                0.10
                            } else if i >= 7 {
                                0.24
                            } else {
                                0.32
                            },
                    },
                );
            }
        }
    }
    fn audible(&self) -> bool {
        !self.muted && matches!(self.output, Output::On)
    }
    pub fn loaded(&self) -> usize {
        self.clips.iter().filter(|s| s.is_some()).count()
    }
}

fn tone(start: f32, end: f32, seconds: f32, arpeggio: bool) -> Vec<u8> {
    let rate = 22050_u32;
    let samples = (seconds * rate as f32) as u32;
    let bytes = samples * 2;
    let mut wav = Vec::with_capacity(bytes as usize + 44);
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(bytes + 36).to_le_bytes());
    wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16_u32.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&rate.to_le_bytes());
    wav.extend_from_slice(&(rate * 2).to_le_bytes());
    wav.extend_from_slice(&2_u16.to_le_bytes());
    wav.extend_from_slice(&16_u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&bytes.to_le_bytes());
    let mut phase = 0.0;
    for i in 0..samples {
        let progress = i as f32 / samples as f32;
        let frequency = if arpeggio {
            [440.0, 554.37, 659.25, 880.0][((progress * 4.0) as usize).min(3)]
        } else {
            start + (end - start) * progress
        };
        phase += std::f32::consts::TAU * frequency / rate as f32;
        let envelope = (progress * 35.0).min(1.0) * (1.0 - progress).powi(2);
        let value = (phase.sin() + 0.18 * (phase * 2.0).sin()) * envelope * 16000.0;
        wav.extend_from_slice(&(value as i16).to_le_bytes());
    }
    wav
}

#[cfg(test)]
mod tests {
    use super::{Audio, Output, START_TIMEOUT, tone};
    use std::{
        sync::mpsc::{Receiver, channel},
        time::Instant,
    };

    fn audio(output: Output) -> Audio {
        Audio {
            clips: std::array::from_fn(|_| None),
            muted: false,
            volume: 1.0,
            output,
        }
    }

    /// A device start that answers `outcome` straight away.
    fn answers(outcome: Result<(), String>) -> Receiver<Result<(), String>> {
        let (ready, receiver) = channel();
        ready.send(outcome).unwrap();
        receiver
    }

    /// Polls as the frame loop would for `frames` frames, `step` apart, and
    /// returns every line it would have logged.
    fn frames(
        output: &mut Output,
        frames: u32,
        step: std::time::Duration,
    ) -> Vec<Result<String, String>> {
        let start = Instant::now();
        (0..frames)
            .filter_map(|i| output.poll(start + step * i))
            .collect()
    }

    #[test]
    fn a_silent_run_never_opens_the_device() {
        let mut opened = false;
        let mut output = Output::begin(true, || {
            opened = true;
            answers(Ok(()))
        });
        assert!(!opened);
        assert!(matches!(output, Output::Off));
        assert!(frames(&mut output, 10, START_TIMEOUT).is_empty());
        let mut audio = audio(output);
        // The frame loop copies the saved setting into `muted` every frame.
        audio.muted = false;
        assert!(!audio.audible());
    }

    #[test]
    fn a_started_device_plays_and_logs_once() {
        let mut opened = false;
        let mut output = Output::begin(false, || {
            opened = true;
            answers(Ok(()))
        });
        assert!(opened);
        let lines = frames(&mut output, 10, START_TIMEOUT);
        assert!(matches!(lines.as_slice(), [Ok(_)]), "{lines:?}");
        let mut audio = audio(output);
        assert!(audio.audible());
        audio.muted = true;
        assert!(!audio.audible());
    }

    #[test]
    fn a_failed_device_start_degrades_to_silent_and_logs_once() {
        let mut output = Output::begin(false, || answers(Err("OSStatus -66681".into())));
        let lines = frames(&mut output, 10, START_TIMEOUT);
        assert!(
            matches!(lines.as_slice(), [Err(line)] if line.contains("OSStatus -66681")),
            "{lines:?}"
        );
        let mut audio = audio(output);
        assert!(!audio.audible());
        // Nothing reaches the mixer, and polling a settled output is quiet.
        audio.play(ark::Events {
            brick: true,
            ..ark::Events::default()
        });
        audio.poll();
        assert!(matches!(audio.output, Output::Failed));
    }

    #[test]
    fn a_device_thread_that_dies_degrades_to_silent() {
        // The start panicked on its thread, dropping the sender unsent.
        let mut output = Output::begin(false, || channel().1);
        let lines = frames(&mut output, 10, START_TIMEOUT);
        assert!(matches!(lines.as_slice(), [Err(_)]), "{lines:?}");
        assert!(!audio(output).audible());
    }

    #[test]
    fn a_hanging_device_start_never_blocks_and_gives_up_once() {
        let (ready, outcome) = channel();
        let mut output = Output::begin(false, || outcome);
        let step = START_TIMEOUT / 30;
        // A minute of frames: silent while it waits, one line at the timeout.
        let lines = frames(&mut output, 30 * 20, step);
        assert!(
            matches!(lines.as_slice(), [Err(line)] if line.contains("did not start")),
            "{lines:?}"
        );
        assert!(matches!(output, Output::Failed));
        // A start that succeeds after the game gave up has nowhere to land.
        assert!(ready.send(Ok(())).is_err());
        assert!(output.poll(Instant::now()).is_none());
        assert!(!audio(output).audible());
    }

    /// The mixer decodes clips on the main thread and panics on a malformed
    /// file, so every synthesized clip must be a well-formed PCM WAV.
    #[test]
    fn synthesized_clips_are_well_formed_wav() {
        for (start, end, seconds, arpeggio) in [
            (640.0, 330.0, 0.07, false),
            (440.0, 1320.0, 0.3, true),
            (1080.0, 810.0, 0.06, false),
            (640.0 * 2.0_f32.powf(7.0 / 12.0), 330.0, 0.07, false),
        ] {
            let wav = tone(start, end, seconds, arpeggio);
            let u32_at = |i: usize| u32::from_le_bytes(wav[i..i + 4].try_into().unwrap());
            let u16_at = |i: usize| u16::from_le_bytes(wav[i..i + 2].try_into().unwrap());
            assert_eq!(&wav[0..4], b"RIFF");
            assert_eq!(u32_at(4) as usize, wav.len() - 8);
            assert_eq!(&wav[8..16], b"WAVEfmt ");
            assert_eq!((u16_at(20), u16_at(22)), (1, 1), "mono PCM");
            assert_eq!(u32_at(28), u32_at(24) * u32::from(u16_at(32)));
            assert_eq!(&wav[36..40], b"data");
            assert_eq!(u32_at(40) as usize, wav.len() - 44);
            assert!(u32_at(40) > 0 && u32_at(40) % 2 == 0);
        }
    }
}
