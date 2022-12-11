import * as sp from "spectrogram-wasm";

let fm: any = null;
let spectrogram: sp.Spectrogram = null;

    sp.init_canvas();
// const initCanvas = () => {
    //     sp.init_canvas();
// }

// const initCanvasButton = document.getElementById("init-canvas");
// initCanvasButton.addEventListener('click', initCanvas)

const startSpectrogram = async () => {
    const mic = await navigator.mediaDevices.getUserMedia({
        audio: true,
    })
    // return mic;
    spectrogram = new sp.Spectrogram(mic);
    // console.log("spectrogram: ", spectrogram);
    // spectrogram.connect_user_mic(mic);
}

const startSpectrogramButton = document.getElementById("start-spectrogram");
startSpectrogramButton.addEventListener('click', startSpectrogram)

const stopSpectrogram = () => {
    if (spectrogram) {
        spectrogram.disconnect_user_mic();
    } else {
        debugger;
    }
}

const stopSpectrogramButton = document.getElementById("stop-spectrogram");
stopSpectrogramButton.addEventListener('click', stopSpectrogram)

const getAnalyserValues = () => {
    spectrogram?.get_frequency_data();
    // let values = new Uint8Array;
    // const analyser = spectrogram.analyser;
    // analyser?.getByteTimeDomainData(values);
    // console.log("values: ", values);
}
const analyserButton = document.getElementById("analyser-values");
analyserButton.addEventListener('click', getAnalyserValues)

const pushAnalyserValues = () => {
    spectrogram?.push_frequency_data();
    // let values = new Uint8Array;
    // const analyser = spectrogram.analyser;
    // analyser?.getByteTimeDomainData(values);
    // console.log("values: ", values);
}
const pushAnalyserButton = document.getElementById("push-analyser-values");
pushAnalyserButton.addEventListener('click', pushAnalyserValues)

// const play_button = document.getElementById("play");
// play_button.addEventListener("click", event => {
//     if (fm === null) {
//     fm = new sp.FmOsc();
//     fm.set_note(50);
//     fm.set_fm_frequency(0);
//     fm.set_fm_amount(0);
//     fm.set_gain(0.8);
//     } else {
//     fm.free();
//     fm = null;
//     }
// });

// interface EventTargetWithValue extends EventTarget {
//     value: string;
// }

// interface InputEventWithValue extends InputEvent {
//     target: EventTargetWithValue;
// }

// const primary_slider: HTMLInputElement = document.getElementById("primary_input") as HTMLInputElement;
// const handleSliderInput = (event: InputEventWithValue) => {
//     if (fm) {
//         fm.set_note(parseInt(event.target.value));
//     }
//     }
// primary_slider.addEventListener("input", handleSliderInput);

// const fm_freq = document.getElementById("fm_freq");
// const handleFmFreq = (event: InputEventWithValue) => {
//     if (fm && event.target.value) {
//         fm.set_fm_frequency(parseFloat(event.target.value));
//     }
//     }
// fm_freq.addEventListener("input", handleFmFreq);

// const fm_amount = document.getElementById("fm_amount");
// const handleFmAmount = (event: InputEventWithValue) => {
//     if (fm) {
//         fm.set_fm_amount(parseFloat(event.target.value));
//     }
//     };

// fm_amount.addEventListener("input", handleFmAmount);