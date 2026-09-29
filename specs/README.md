# Specs

What the game does, one file per area. `TEMPLATE.md` is the starting point for a
new one, and `CLAUDE.md` in the repo root describes the flow.

Specs are numbered in the order they were written. The number is an identifier,
not a priority, and it never changes once a spec exists.

| Spec | Covers |
| --- | --- |
| [0001](0001-the-tunnel.md) | The tube, where it wanders, and its wall |
| [0002](0002-the-ship.md) | Steering by mouse, and being held in |
| [0003](0003-rings.md) | Where to steer to, and threading them |
| [0004](0004-the-run.md) | The number at the end, and flying it again |

The tunnel shares its shape and its numbers with blitzkit's `tunnel` example.
Neither repo can see the other, so `check-tunnel` in the project folder is the
only place the two are held to the same story. Changing a number in 0001 means
running it.
