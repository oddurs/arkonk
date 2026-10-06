use arkonk::game::Events;
use macroquad::audio::{PlaySoundParams, Sound, load_sound_from_bytes, play_sound};

pub struct Audio {
    clips: [Option<Sound>; 7],
    pub muted: bool,
}
impl Audio {
    pub async fn new() -> Self {
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
            let wave = tone(start, end, seconds);
            clips[i] = load_sound_from_bytes(&wave).await.ok();
        }
        Self {
            clips,
            muted: false,
        }
    }
    pub fn play(&self, e: Events) {
        if self.muted {
            return;
        }
        for (i, enabled) in [
            e.brick, e.paddle, e.wall, e.lost, e.clear, e.pickup, e.launch,
        ]
        .into_iter()
        .enumerate()
        {
            if enabled && let Some(sound) = &self.clips[i] {
                play_sound(
                    sound,
                    PlaySoundParams {
                        looped: false,
                        volume: 0.22,
                    },
                );
            }
        }
    }
    pub fn loaded(&self) -> usize {
        self.clips.iter().filter(|s| s.is_some()).count()
    }
}

fn tone(start: f32, end: f32, seconds: f32) -> Vec<u8> {
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
        phase += std::f32::consts::TAU * (start + (end - start) * progress) / rate as f32;
        let envelope = (progress * 35.0).min(1.0) * (1.0 - progress).powi(2);
        let value = (phase.sin() + 0.18 * (phase * 2.0).sin()) * envelope * 16000.0;
        wav.extend_from_slice(&(value as i16).to_le_bytes());
    }
    wav
}
