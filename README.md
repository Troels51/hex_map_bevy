## Hex map in Bevy
[![CI](https://github.com/Troels51/hex_map_bevy/workflows/CI/badge.svg?branch=master)](https://github.com/Troels51/hex_map_bevy/actions?query=workflow%3A%22CI%22+branch%3Amaster)

This is a small 3D application with a hex map. Hex art by Andreas Barbesgaard

- Press F to change to the next hex model
- Press R to rotate the selected hex on the map
- WASD to move the camera

![Preview](/assets/hex_map.PNG)


## How to run
 1. Install Rust (https://www.rust-lang.org/tools/install)
 4. Run the application
    * Start the native app: `cargo run`
    * Start the web build: `cargo run --target wasm32-unknown-unknown`
       * requires [`wasm-server-runner`]: `cargo install wasm-server-runner`
       * requires `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
       * this will serve your app on a free port

## How to generate Hex models
The script batch_export under models/blender_files will generate a 3D model for each hex texture. Modify and run the script in blender if you modify the hex images.
