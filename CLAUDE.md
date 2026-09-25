# slider

Fly down a tunnel that wanders, through fourteen rings strung along it. The
fifth game on `blitzkit` and the second in 3D. The engine owns the window,
rendering, input and the surfaces both the tunnel and the rings are built from.
The dependency is the published crate, overridden by the engine checkout at
`../blitzkit` when built inside this project folder.

## Build and test

Requires Rust 1.87 or newer, the engine's MSRV.

```
cargo build
cargo test
cargo run
cargo clippy
cargo fmt
```

## How work happens here

Behavior changes are spec driven:

1. **Write the spec first.** Copy `specs/TEMPLATE.md` to `specs/NNNN-short-name.md`
   and fill it in. Say what the game does, not how the code does it.
2. **Make the acceptance criteria testable.** Each one names the test that proves
   it, or goes under "Verified by hand" when it needs a window.
3. **Write the tests, then the code.** `cargo test` passes before a commit.
4. **Update the spec when behavior changes.** A spec that disagrees with the game
   is a bug in the spec.

## Layout

- `src/main.rs` — hands a `SliderGame` to `blitzkit::start`.
- `src/slider_game.rs` — the `Game` impl, the drawing and the readout.
- `src/tunnel.rs` — where the tunnel runs, and where its wall is.
- `src/ring.rs` — where the rings are, and what counts as through one.
- `src/ship.rs` — steering, drift, speed, and what the wall does to you.
- `src/run.rs` — judging the rings as they go by, and ending at the far end.
- `src/input.rs` — the mouse, the cursor, the keys.
- `src/checker.rs` — the wall's checker, built rather than loaded. No image
  files travel with this game.

## Where it came from

It began as `blitzkit/examples/tunnel.rs`, which is still there and still worth
running: it is the hand-check for spec 0016's two-sided geometry and spec 0011's
mipmaps. This is the same idea with the game around it and the maths under test.

The two share about thirty lines of tube formula and are free to drift apart.
The example should stay the smallest thing that demonstrates the engine; this
one can grow.

Writing the tests here found three bugs that had already shipped in that
example: three rings embedded in the tunnel wall, a quarter of the tunnel that
never got a ring, and six of fourteen rings takeable without steering at all.
All three came from one line that multiplied a vector by a constant without
checking how long the vector could get.
