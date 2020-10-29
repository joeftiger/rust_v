# Rust\-_V_
A Rust-written ray tracer for my B.Sc. thesis.

The _V_ stands for a ray reflection, and therefore figuratively for ray tracing.

# Building
## Cargo
On the first build, _Cargo_ will need to download some crates as dependencies, just sit tight and wait a while. \
Run: \
`$  cargo build --package rust_v --bin rust_v`

For a release (optimized) version, append `--release`: \
`$  cargo build --package rust_v --bin rust_v --release`

If you want to run the program immediately (help), use \
`$  cargo run --package rust_v --bin rust_v -- --help`


## Jetbrains IDEs
1. Install the `rust` plugin
2. Import any of the given run configurations in `$PROJECT_DIR$/.idea/runConfigurations/`
3. Build / Run (and have fun ;-)
