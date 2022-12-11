#![allow(dead_code)]
// mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, AnalyserNode, MediaStream, MediaStreamAudioSourceNode, MediaStreamAudioSourceOptions, console};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct Spectrogram {
    ctx: AudioContext,

    /// Overall gain (volume) control
    gain: web_sys::GainNode,

    // Analyser node for getting FFT outputs
    analyser: web_sys::AnalyserNode,

    // Audio source, typically user's microphone
    source: web_sys::MediaStreamAudioSourceNode,

    // Frequency domain values from analyser node
    frequency_data: Vec<Vec<u8>>,
}

#[wasm_bindgen]
impl Spectrogram {
    #[wasm_bindgen(constructor)]
    pub fn new(stream: MediaStream) -> Result<Spectrogram, JsValue> {
        let ctx = web_sys::AudioContext::new()?;
        // let ctx = web_sys::AudioContext::new_with_context_options({44100 })?;

        // Create our web audio objects.
        let gain = ctx.create_gain()?;
        let analyser = web_sys::AnalyserNode::new(&ctx)?;

        // Some initial settings:
        gain.gain().set_value(20.0);

        let source = ctx.create_media_stream_source(&stream)?;
        // The audio source is routed through the gain node, so that
        // it can control the overall output volume.
        source.connect_with_audio_node(&gain)?
        .connect_with_audio_node(&analyser)
        .expect("analyser connected");

        // Then connect the gain node to the AudioContext destination (aka
        // your speakers).
        // gain.connect_with_audio_node(&ctx.destination())?;

        let frequency_data: Vec<Vec<u8>> = vec![vec![0;128]];

        Ok(Spectrogram {
            ctx,
            gain,
            analyser,
            source,
            frequency_data,
        })
    }

    pub fn disconnect_user_mic(&self) {
        let _ = &self.source.disconnect();
    }

    pub fn push_frequency_data(&mut self) {
        let mut sample: Vec<u8> = vec![0; 128];
        self.analyser.get_byte_frequency_data(&mut sample[0..]);
        if self.frequency_data.len() < 100 {
            self.frequency_data.push(sample);
        }
    }

    pub fn get_frequency_data(&mut self) {
        log!("Frequency data length: {}", self.frequency_data.len());
        log!("Frequency data: {:?}", self.frequency_data);
    }
}