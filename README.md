## TetRS
---

A Tetris clone written in Rust.

### Why another Tetris clone?

The idea here is to create a "full featured" Tetris clone. Abstractions exist within the codebase that pave the way for future updates to extend the core mechanics.

## Building and running

Via cargo:

```
cargo run
```

### How to play:

Controls are:

* Arrow keys for movement.
* `Z` for counter-clockwise Tetromino rotation.
* `X` for clockwise Tetromino rotation.
* `Space` to drop a Tetromino immediately.
* `Escape` pauses the game.

### Shadow

The game supports a "Shadow" toggle to preview where a Tetromino will drop. You can toggle this via the `--shadow-enabled` flag:

```
cargo run -- --shadow-enabled
```

![tetrs-shadow](https://cloud.githubusercontent.com/assets/2499070/20560310/c501ceb8-b1cc-11e6-8dbc-056f7e489592.gif)
