# Rust\-_V_
A Rust-written ray tracer for my B.Sc. thesis.

The _V_ stands for a ray reflection, and therefore figuratively for ray tracing.

## Building
### Features
#### `live-window`
By passing `--live` as runtime argument, the rendering will open in a window , showing you the progress. \

The window allows you some commands like following:
- <strike>`Backspace`: Reset render progress (start again)</strike>
- <strike>`Enter`: Save current rendering as 16-bit PNG</strike>
- `Ctrl + s`: Save current rendering as 8-bit PNG (with GUI ;-)
- <strike>`←↑→↓` (Arrow keys): Rotate camera</strike>

NOTE: Due to concurrency complexity, our `FastWindow` currently does not allow custom commands.
Maybe we re-implement it later, or you can crate a pull request :-)

### Cargo
On the first build, _Cargo_ will need to download some crates as dependencies, just sit tight and wait a while. \
Run: \
`$  cargo build --package rust_v --bin rust_v`

For a release (optimized) version, append `--release`: \
`$  cargo build --package rust_v --bin rust_v --release`

For a live-window enabled version, append `--features "live-window"`: \
`$  cargo build --package rust_v --bin rust_v --features "live-window"`

The compiled binary should be in the folder `${RUST_V}/target/(dev|release)/rust_v`

## Progress
I currently keep track of my progress on [Trello](https://trello.com/b/hMhdBrAU/rust-v). I try to keep it up-to-date,
but don't throw bricks at me if I forgot something. :)
