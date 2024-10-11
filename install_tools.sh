#!/bin/bash

# The complication target for browser-based WebAssembly is wasm32-unknown-unknown target
rustup target add wasm32-unknown-unknown

# Install tools
cargo install --locked trunk
