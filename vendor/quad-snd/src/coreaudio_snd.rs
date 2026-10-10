use crate::PlaySoundParams;

use std::sync::mpsc;

pub use crate::mixer::Playback;

#[path = "coreaudio/coreaudio.rs"]
mod coreaudio;

// bindgen lost some defines from coreaudio.rs
const _saudio_kAudioFormatLinearPCM: u32 = 1819304813; //'lpcm';
const _saudio_kLinearPCMFormatFlagIsFloat: u32 = 1 << 0;
const _saudio_kAudioFormatFlagIsPacked: u32 = 1 << 3;

use coreaudio::*;

mod consts {
    pub const DEVICE: &'static str = "default\0";
    pub const RATE: u32 = 44100;
    pub const CHANNELS: u32 = 2;
    pub const BUFFER_FRAMES: u32 = 4096;
}

pub struct AudioContext {
    pub(crate) mixer_ctrl: crate::mixer::MixerControl,
}

unsafe extern "C" fn saudio_coreaudio_callback(
    user_data: *mut ::std::os::raw::c_void,
    queue: _saudio_AudioQueueRef,
    buffer: _saudio_AudioQueueBufferRef,
) {
    let mut mixer = &mut *(user_data as *mut crate::mixer::Mixer);

    let num_frames = (*buffer).mAudioDataByteSize / (2 * 4);
    let buf =
        std::slice::from_raw_parts_mut((*buffer).mAudioData as *mut f32, num_frames as usize * 2);

    mixer.fill_audio_buffer(buf, num_frames as _);

    AudioQueueEnqueueBuffer(queue, buffer, 0, std::ptr::null_mut());
}

impl AudioContext {
    pub fn new() -> AudioContext {
        use crate::mixer::Mixer;

        let (mixer_builder, mixer_ctrl) = Mixer::new();
        // ARKONK: deferred to `start_output`; see `output` in lib.rs.
        crate::output::park(Box::new(move |ready| {
            let _ = ready.send(unsafe { start_queue(mixer_builder.build()) });
        }));

        AudioContext { mixer_ctrl }
    }
}

/// ARKONK: the upstream body of `AudioContext::new`, with each `assert!`
/// turned into an error. `AudioQueueStart` can block for many seconds and then
/// fail when coreaudiod is wedged, so this runs on its own thread.
///
/// The mixer is never freed, even after a failure: it holds the receiving end
/// of `MixerControl`'s channel, and dropping it would make every later send
/// print "Audio thread died".
unsafe fn start_queue(mixer: crate::mixer::Mixer) -> Result<(), String> {
    let mixer = Box::into_raw(Box::new(mixer));
    let fmt = _saudio_AudioStreamBasicDescription {
        mSampleRate: consts::RATE as f64,
        mFormatID: _saudio_kAudioFormatLinearPCM,
        mFormatFlags: _saudio_kLinearPCMFormatFlagIsFloat | _saudio_kAudioFormatFlagIsPacked,
        mFramesPerPacket: 1,
        mChannelsPerFrame: consts::CHANNELS,
        mBytesPerFrame: 4 * consts::CHANNELS,
        mBytesPerPacket: 4 * consts::CHANNELS,
        mBitsPerChannel: 32,
        mReserved: 0,
    };
    let mut ca_audio_queue: _saudio_AudioQueueRef = std::mem::zeroed();
    let res = AudioQueueNewOutput(
        &fmt,
        Some(saudio_coreaudio_callback),
        mixer as *mut _,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        0,
        &mut ca_audio_queue,
    );
    if res != 0 || ca_audio_queue.is_null() {
        return Err(format!("AudioQueueNewOutput failed (OSStatus {res})"));
    }

    // create 2 audio buffers
    for _ in 0..2 {
        let mut buf: _saudio_AudioQueueBufferRef = std::ptr::null_mut();
        let buf_byte_size = consts::BUFFER_FRAMES * fmt.mBytesPerFrame;
        let res = AudioQueueAllocateBuffer(ca_audio_queue, buf_byte_size, &mut buf);
        if res != 0 || buf.is_null() {
            return Err(dispose(ca_audio_queue, "AudioQueueAllocateBuffer", res));
        }
        (*buf).mAudioDataByteSize = buf_byte_size;
        std::ptr::write_bytes(
            (*buf).mAudioData as *mut u8,
            0,
            (*buf).mAudioDataByteSize as usize,
        );
        AudioQueueEnqueueBuffer(ca_audio_queue, buf, 0, std::ptr::null_mut());
    }

    let res = AudioQueueStart(ca_audio_queue, std::ptr::null_mut());
    if res != 0 {
        return Err(dispose(ca_audio_queue, "AudioQueueStart", res));
    }
    Ok(())
}

/// Frees a queue that never started and names the call that failed.
unsafe fn dispose(queue: _saudio_AudioQueueRef, call: &str, res: _saudio_OSStatus) -> String {
    AudioQueueDispose(queue, true);
    format!("{call} failed (OSStatus {res})")
}

pub struct Sound {
    sound_id: u32,
}

impl Sound {
    pub fn load(ctx: &AudioContext, data: &[u8]) -> Sound {
        let sound_id = ctx.mixer_ctrl.load(data);

        Sound { sound_id }
    }

    pub fn play(&self, ctx: &AudioContext, params: PlaySoundParams) -> Playback {
        ctx.mixer_ctrl.play(self.sound_id, params)
    }

    pub fn stop(&self, ctx: &AudioContext) {
        ctx.mixer_ctrl.stop_all(self.sound_id);
    }

    pub fn set_volume(&self, ctx: &AudioContext, volume: f32) {
        ctx.mixer_ctrl.set_volume_all(self.sound_id, volume);
    }

    pub fn delete(&self, ctx: &AudioContext) {
        ctx.mixer_ctrl.delete(self.sound_id);
    }
}
