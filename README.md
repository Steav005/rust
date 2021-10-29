# Rust Toolchain for L4Re

## Supported Targets
- **x86_64**: `x86_64-unknown-l4re-uclibc`
- **armv7**: `armv7-unknown-l4re-uclibceabihf`

## Installation

1. Make sure all dependencies are installed (See [#Building on a Unix-like system](https://github.com/rust-lang/rust/tree/c8dfcfe046a7680554bf4eb612bad840e7631c4b#building-on-a-unix-like-system))
2. Change `config.toml` according to your build-system and preferences (Current configuration is made for x86_64 build-system, offering all supported l4re targets)
3. Run `./x.py build` in project-root directory
4. Run `./x.py install --stage 1 --keep-stage 0 --keep-stage 1`
5. Now link the toolchain to rustup (rustup must be installed)  
    `rustup toolchain link l4re-1.55.0-stable /path/to/this/projects/folder/patched/`
6. (Optional) Mark the `l4re-1.55.0-stable` toolchain as the default toolchain  
    `rustup default l4re-1.55.0-stable`
