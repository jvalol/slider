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

## The tunnel, and the other thing built on it

`blitzkit/examples/tunnel.rs` builds the same tunnel this does. Not an ancestor
of it: a sibling. Both build a wandering tube with rings in it, and neither is
derived from the other.

They cannot share a line of code. This game takes blitzkit from crates.io the
way anyone else would, and the tunnel is not in the engine: it is not a shape in
`blitzkit-shapes` either, because that is `publish = false` and so out of reach
from out here. So the thirty lines of tube formula are written twice on purpose,
and `check-tunnel` in the project folder is the only thing holding the two sets
of numbers together.

They answer different questions, which is why both exist:

- The example asks whether an engine feature works. It is the hand-check for
  spec 0016's two-sided geometry and spec 0011's mipmaps, and it has to stay the
  smallest thing that shows them, so that a failure points at the engine rather
  than at whatever else was built around it.
- This asks whether the published engine is usable by someone outside it. Seven
  modules, its own specs, its own tests, and it is free to grow, which the
  example specifically is not.

Delete the example and specs 0016 and 0011 have nothing to run. Delete this and
nothing checks that the crate on crates.io works for a stranger.

Writing the tests here found three bugs that had already shipped in that
example: three rings embedded in the tunnel wall, a quarter of the tunnel that
never got a ring, and six of fourteen rings takeable without steering at all.
All three came from one line that multiplied a vector by a constant without
checking how long the vector could get.
