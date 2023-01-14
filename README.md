# The Rosetta Block Trials 

This is a minimalistic recreation of the puzzle game: [[7MMC] The Rosetta Block Trials` by rubenscube](https://mmcelebration.com/level/4/31/#!/about). 
Using [Yew](https://yew.rs/) and built with [Trunk](https://github.com/thedodd/trunk).

## Credit

- Puzzle by: [rubenscube](https://mmcelebration.com/level/4/31/#!/about)
- Tech: [Yew](https://yew.rs/)+ [Trunk](https://github.com/thedodd/trunk) + [Rust](https://www.rust-lang.org/)
- Colors: [Rosé Pine](https://rosepinetheme.com)
- Programer: [varpeti](https://github.com/varpeti)

## Demo

- [Itch.io](https://itep.itch.io/the-rosetta-block-trials)

### Installation

If you don't already have it installed, it's time to install Rust: <https://www.rust-lang.org/tools/install>.
The rest of this guide assumes a typical Rust installation which contains both `rustup` and Cargo.

To compile Rust to WASM, we need to have the `wasm32-unknown-unknown` target installed.
If you don't already have it, install it with the following command:

```bash
rustup target add wasm32-unknown-unknown
```

Now that we have our basics covered, it's time to install Trunk.
Simply run the following command to install it:

```bash
cargo install trunk wasm-bindgen-cli
```

That's it, we're done!

### Running

```bash
trunk serve
```

Rebuilds the app whenever a change is detected and runs a local server to host it.

There's also the `trunk watch` command which does the same thing but without hosting it.

### Release

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`.
You can also pass the `--release` flag to `trunk serve` if you need to get every last drop of performance.

Unless overwritten, the output will be located in the `dist` directory.

### License

MIT license.
