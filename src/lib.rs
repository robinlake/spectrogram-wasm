#![allow(dead_code)]
mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, OscillatorType, AnalyserNode, MediaStream, MediaStreamAudioSourceNode, MediaStreamAudioSourceOptions, console};

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
        gain.gain().set_value(100.0);

        let source = ctx.create_media_stream_source(&stream)?;
        // The audio source is routed through the gain node, so that
        // it can control the overall output volume.
        source.connect_with_audio_node(&gain)?
        .connect_with_audio_node(&analyser)
        .expect("analyser connected");

        // Then connect the gain node to the AudioContext destination (aka
        // your speakers).
        gain.connect_with_audio_node(&ctx.destination())?;

        Ok(Spectrogram {
            ctx,
            gain,
            analyser,
            source,
        })
    }

    pub fn disconnect_user_mic(&self) {
        let _ = &self.source.disconnect();
    }

    pub fn get_frequency_data(&self) {
        // let sampleRate: &f32 = &self.ctx.sample_rate();
        // let valueArr: [Vec<i8>; sampleRate] = [];
        // let mut frequency_data = Vec::new();
        let mut frequency_data: [u8; 44100] = [5; 44100];
        // let frequency_data: &mut [u8] = &mut[];
        // let mut frequency_data = Vec::new();
        self.analyser.get_byte_frequency_data(&mut frequency_data[0..44099]);
        log!("Frequency data foo: {:?}", frequency_data);
    }
}

/*
 Copied from https://rustwasm.github.io/wasm-bindgen/examples/web-audio.html
 */
/// Converts a midi note to frequency
///
/// A midi note is an integer, generally in the range of 21 to 108
pub fn midi_to_freq(note: u8) -> f32 {
    27.5 * 2f32.powf((note as f32 - 21.0) / 12.0)
}

#[wasm_bindgen]
pub struct FmOsc {
    ctx: AudioContext,
    /// The primary oscillator.  This will be the fundamental frequency
    primary: web_sys::OscillatorNode,

    /// Overall gain (volume) control
    gain: web_sys::GainNode,

    /// Amount of frequency modulation
    fm_gain: web_sys::GainNode,

    /// The oscillator that will modulate the primary oscillator's frequency
    fm_osc: web_sys::OscillatorNode,

    /// The ratio between the primary frequency and the fm_osc frequency.
    ///
    /// Generally fractional values like 1/2 or 1/4 sound best
    fm_freq_ratio: f32,

    fm_gain_ratio: f32,
}

impl Drop for FmOsc {
    fn drop(&mut self) {
        let _ = self.ctx.close();
    }
}

#[wasm_bindgen]
impl FmOsc {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<FmOsc, JsValue> {
        let ctx = web_sys::AudioContext::new()?;

        // Create our web audio objects.
        let primary = ctx.create_oscillator()?;
        let fm_osc = ctx.create_oscillator()?;
        let gain = ctx.create_gain()?;
        let fm_gain = ctx.create_gain()?;

        // Some initial settings:
        primary.set_type(OscillatorType::Sine);
        primary.frequency().set_value(440.0); // A4 note
        gain.gain().set_value(0.0); // starts muted
        fm_gain.gain().set_value(0.0); // no initial frequency modulation
        fm_osc.set_type(OscillatorType::Sine);
        fm_osc.frequency().set_value(0.0);

        // Connect the nodes up!

        // The primary oscillator is routed through the gain node, so that
        // it can control the overall output volume.
        primary.connect_with_audio_node(&gain)?;

        // Then connect the gain node to the AudioContext destination (aka
        // your speakers).
        gain.connect_with_audio_node(&ctx.destination())?;

        // The FM oscillator is connected to its own gain node, so it can
        // control the amount of modulation.
        fm_osc.connect_with_audio_node(&fm_gain)?;

        // Connect the FM oscillator to the frequency parameter of the main
        // oscillator, so that the FM node can modulate its frequency.
        fm_gain.connect_with_audio_param(&primary.frequency())?;

        // Start the oscillators!
        primary.start()?;
        fm_osc.start()?;

        Ok(FmOsc {
            ctx,
            primary,
            gain,
            fm_gain,
            fm_osc,
            fm_freq_ratio: 0.0,
            fm_gain_ratio: 0.0,
        })
    }

    /// Sets the gain for this oscillator, between 0.0 and 1.0.
    #[wasm_bindgen]
    pub fn set_gain(&self, mut gain: f32) {
        if gain > 1.0 {
            gain = 1.0;
        }
        if gain < 0.0 {
            gain = 0.0;
        }
        self.gain.gain().set_value(gain);
    }

    #[wasm_bindgen]
    pub fn set_primary_frequency(&self, freq: f32) {
        self.primary.frequency().set_value(freq);

        // The frequency of the FM oscillator depends on the frequency of the
        // primary oscillator, so we update the frequency of both in this method.
        self.fm_osc.frequency().set_value(self.fm_freq_ratio * freq);
        self.fm_gain.gain().set_value(self.fm_gain_ratio * freq);
    }

    #[wasm_bindgen]
    pub fn set_note(&self, note: u8) {
        let freq = midi_to_freq(note);
        self.set_primary_frequency(freq);
    }

    /// This should be between 0 and 1, though higher values are accepted.
    #[wasm_bindgen]
    pub fn set_fm_amount(&mut self, amt: f32) {
        self.fm_gain_ratio = amt;

        self.fm_gain
            .gain()
            .set_value(self.fm_gain_ratio * self.primary.frequency().value());
    }

    /// This should be between 0 and 1, though higher values are accepted.
    #[wasm_bindgen]
    pub fn set_fm_frequency(&mut self, amt: f32) {
        self.fm_freq_ratio = amt;
        self.fm_osc
            .frequency()
            .set_value(self.fm_freq_ratio * self.primary.frequency().value());
    }
}
