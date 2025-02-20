# crrshrr

This **crrshrr** is a simpler bit crusher effect than the **crrshrr** it is based off of. It is available as a VST and CLAP plugin for Windows and Linux, and can be compiled to Windows, Linux and OSX.

There is a "CRUNCH" button which switches the algorithm used for the downsampling. No crunch can only downsample at certain thresholds, but results in a different, cleaner sound. With crunch on, you get a noisier result, but can freely use whatever samplerate you like.  

The "GATE" button determines whether sound at the noise floor (which depends on the bit depth) is rounded up or down. At lower bit depth settings this results in a noticeable "gating" effect.

This was created as a way to play around with Rust, [nih-plug](https://github.com/robbert-vdh/nih-plug), and audio effects development in general.

## Building

After installing [Rust](https://rustup.rs/), you can compile crrshrr as follows:

```shell
cargo xtask bundle crrshrr --release
```
