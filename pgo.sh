#!/bin/bash

export RUSTFLAGS="-Ctarget-cpu=native -Cprofile-generate=/tmp/pgo-data"

cargo run --package rust_v --bin rust_v --release -- demo -w 400 -h 400 -d 3 -p 1 -i path --block-size 8 --threaded -o ""
cargo run --package rust_v --bin rust_v --release -- demo -w 300 -h 300 -d 6 -p 2 -i path --block-size 8 --threaded -o ""
cargo run --package rust_v --bin rust_v --release -- demo -w 200 -h 200 -d 10 -p 4 -i path --block-size 8 --threaded -o ""
cargo run --package rust_v --bin rust_v --release -- demo -w 500 -h 500 -d 10 -p 8 -i path --block-size 8 --threaded -o ""

$HOME/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata merge -o /tmp/pgo-data.profdata /tmp/pgo-data

export RUSTFLAGS="-Ctarget-cpu=native -Cprofile-use=/tmp/pgo-data.profdata"
cargo build --package rust_v --bin rust_v --release
