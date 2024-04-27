#!/bin/bash

PROJECT_NAME="bevy_glicol_wasm"

cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --no-typescript --target web --out-dir ./ --out-name "$PROJECT_NAME" ./target/wasm32-unknown-unknown/release/$PROJECT_NAME.wasm