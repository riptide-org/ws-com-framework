#!/usr/bin/env bash
# Author: Jo Bull, 2021
# A small script to automatically test the full feature set of this crate - as this is not yet implemented in rust.
# This issue is tracked here: https://github.com/rust-lang/cargo/issues/2911

cargo test --lib ./macros;
read -n 1 -s -r -p "Press any key to continue"
cargo test --no-default-features --features "client, wrapper-tungstenite, wrapper-tokio, wrapper-websocket, wrapper-warp"
read -n 1 -s -r -p "Press any key to continue"
cargo test --no-default-features --features "server, wrapper-tungstenite, wrapper-tokio, wrapper-websocket, wrapper-warp"