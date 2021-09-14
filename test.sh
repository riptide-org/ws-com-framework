#!/usr/bin/env bash
# A small script to automatically test the full feature set of this crate - as this is not yet implemented in rust.
# This issue is tracked here: https://github.com/rust-lang/cargo/issues/2911
# Author: Jo Bull

cargo test --lib ./macros;
read -n 1 -s -r -p "Press any key to continue"
cargo test --features client;
read -n 1 -s -r -p "Press any key to continue"
cargo test --features server;