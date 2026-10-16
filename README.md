# Bevyworks

Learning rust, and trying to create fireworks using Bevy game engine.

The current demo already launches fireworks automatically: randomized shells rise
under gravity, explode on a timer into colored particles, and are removed when
their lifetimes expire. It is a learning experiment, not a finished effects
library or game. Mouse-directed launch code is commented out; particle fading
remains a TODO.

## Run

`Cargo.toml` uses Rust edition **2018**, Bevy **0.4**, and rand **0.8.0**. This is
a legacy Bevy stack, not code for the current engine API. No Rust toolchain
version is pinned. Use Rust/Cargo and native graphics/windowing dependencies
compatible with Bevy 0.4, on a desktop capable of opening a rendered window.

From the repository root:

```sh
cargo run
```

This follows the committed Cargo entrypoint; compatibility with a modern host
has not been build/run tested for this documentation. Keep the checked-in
`Cargo.lock` when investigating reproducibility rather than assuming a dependency
upgrade is harmless.

## Source

`src/main.rs` contains the whole demo: `launcher` creates shells,
`firework_update` applies gravity/motion, `explode` handles expiration, and `boom`
spawns the explosion particles. `mouse_movement_detector` records cursor position,
but the launcher does not currently use it.
