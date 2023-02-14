#!/bin/bash

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add the ARMv7 target
rustup target add armv7-unknown-linux-gnueabihf

# Create a new Rust project
cargo init