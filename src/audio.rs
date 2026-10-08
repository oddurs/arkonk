use ark::Events;
use macroquad::audio::{PlaySoundParams, Sound, load_sound_from_bytes, play_sound};

pub struct Audio {
    clips: [Option<Sound>; 17],
    pub muted: bool,
    pub volume: f32,
    // Apart from `muted`, which follows the player's setting every frame.
    silent: bool,
}
impl Audio {
    /// `silent` still decodes every clip, so a test run checks they load.
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
            silent,
        };
        crate::diagnostics::info(format_args!("Sounds loaded: {}/17", audio.loaded()));
        audio
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
        !self.muted && !self.silent
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
    use super::{Audio, tone};

    #[test]
    fn a_silent_run_stays_silent_when_settings_unmute() {
        let mut audio = Audio {
            clips: std::array::from_fn(|_| None),
            muted: true,
            volume: 1.0,
            silent: true,
        };
        // The frame loop copies the saved setting into `muted` every frame.
        audio.muted = false;
        assert!(!audio.audible());
        audio.silent = false;
        assert!(audio.audible());
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
