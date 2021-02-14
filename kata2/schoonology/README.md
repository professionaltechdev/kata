# Langton's Ant

- **Author** - Michael Schoonmaker, [@Schoonology][twitter]
- **Language** - Rust

## Running the Kata

1. Install Rust.
1. `cargo run`.
1. Enjoy! Press `q` at any time to quit.

## Implementation notes

- This implementation uses [rustbox][rustbox] to draw the grid.
- You can change the STEP_HZ value in the code to make it render faster or slower.
- The width and height of the grid are set by the width and height of the shell it is run in. If you want to reset to the default 11x11 grid, update the invocation of `Game::new` to `Game::new(11, 11)`.

[twitter]: https://twitter.com/Schoonology
[rustbox]: https://github.com/gchp/rustbox
