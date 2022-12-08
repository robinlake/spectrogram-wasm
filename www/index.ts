import * as wasm from "hello-wasm-pack";

wasm.greet();

alert("this is a new message");

import('../pkg')
  .then(rust_module => {
    let fm: any = null;

    const play_button = document.getElementById("play");
    play_button.addEventListener("click", event => {
      if (fm === null) {
        fm = new rust_module.FmOsc();
        fm.set_note(50);
        fm.set_fm_frequency(0);
        fm.set_fm_amount(0);
        fm.set_gain(0.8);
      } else {
        fm.free();
        fm = null;
      }
    });

    interface EventTargetWithValue extends EventTarget {
        value: string;
    }

    interface InputEventWithValue extends InputEvent {
        target: EventTargetWithValue;
    }

    const primary_slider: HTMLInputElement = document.getElementById("primary_input") as HTMLInputElement;
    const handleSliderInput = (event: InputEventWithValue) => {
        if (fm) {
          fm.set_note(parseInt(event.target.value));
        }
      }
    primary_slider.addEventListener("input", handleSliderInput);

    const fm_freq = document.getElementById("fm_freq");
    const handleFmFreq = (event: InputEventWithValue) => {
        if (fm && event.target.value) {
          fm.set_fm_frequency(parseFloat(event.target.value));
        }
      }
    fm_freq.addEventListener("input", handleFmFreq);

    const fm_amount = document.getElementById("fm_amount");
    const handleFmAmount = (event: InputEventWithValue) => {
        if (fm) {
          fm.set_fm_amount(parseFloat(event.target.value));
        }
      };
    
    fm_amount.addEventListener("input", handleFmAmount);

  })
  .catch(console.error);