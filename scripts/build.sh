#!/bin/bash
export RUSTFLAGS="-C opt-level=3 -C debuginfo=0 -C target-cpu=native"
cargo build --release
