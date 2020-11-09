# Rust\-_V_
A Rust-written ray tracer for my B.Sc. thesis.

The _V_ stands for a ray reflection, and therefore figuratively for ray tracing.

## Building
### Features
#### `live-window`
By passing `--live` as runtime argument, the rendering will open in a window , showing you the progress. \
The window allows you some commands like following:
- `Backspace`: Reset render progress (start again)
- `Enter`: Save current rendering as 16-bit PNG (hardcoded path to `./rendering.png` for now, sorry!)
- `Ctrl + s`: Save current rendering as 8-bit PNG (with GUI ;-)
- `←↑→↓` (Arrow keys): Rotate camera

### Cargo
On the first build, _Cargo_ will need to download some crates as dependencies, just sit tight and wait a while. \
Run: \
`$  cargo build --package rust_v --bin rust_v`

For a release (optimized) version, append `--release`: \
`$  cargo build --package rust_v --bin rust_v --release`

If you want to run the program immediately (help), use \
`$  cargo run --package rust_v --bin rust_v -- --help`


### Jetbrains IDEs
1. Install the `rust` plugin
2. Import any of the given run configurations in `$PROJECT_DIR$/.idea/runConfigurations/`
3. Build / Run (and have fun ;-)

## _Live_ View
By passing `--live` as runtime argument, the rendering will open in a window , showing you the progress. \
The window allows you some commands like following:
- `Backspace`: Reset render progress (start again)
- `Enter`: Save current rendering as 16-bit PNG (hardcoded path to `./rendering.png` for now, sorry!)
- `Ctrl + s`: Save current rendering as 8-bit PNG (with GUI ;-)
- `←↑→↓` (Arrow keys): Rotate camera

## Progress
I currently keep track of my progress on [Trello](https://trello.com/b/hMhdBrAU/rust-v). I try to keep it up-to-date,
but don't throw bricks at me if I forgot something. :)
