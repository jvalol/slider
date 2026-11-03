# 0005 A picture of the tunnel from inside

**Status:** implemented
**Date:** 2026-11-03

## Goal

The frame slider puts on its readme is held against a picture of what that
frame should be, so a change to the tunnel, the checker or the rings is seen
rather than shipped.

## Why it belongs here

Spec 0001 says what the tunnel is, 0004 says what a run through it does, and
every test of either is arithmetic: a position along the nine hundred
metres, a bend, a ring passed. None of them can see the tunnel. The one
check that comes close is `./check-tunnel`, and that holds slider's twenty
numbers against the engine's tunnel example rather than against a picture.

The engine's spec 0076 made the rest of it one call, so what a game writes is
the reference, the size and what to draw.

**And the staging was already here.** `--screenshot` flies the ship in for
`FLOWN_FOR` seconds and then stops the clock, because the opening frame is
the mouth of the tunnel, straight and empty. Twelve point three seconds puts
it about two hundred and seventy five metres in, where the tunnel is bending
and the ring at three hundred hangs ahead. That is the frame
`refresh-screenshots` photographs, so this holds the picture that is already
on show.

## Behavior

**`SliderGame::new` takes `staged` rather than reading the arguments.** A
picture test asks for the photographed state without `--screenshot` in its
own command line. `main` passes what the arguments say, which is the only
place that reads them.

**One `update`, at no time at all.** The staged branch runs its own fixed
step until `flown` reaches `FLOWN_FOR` and then forces `dt` to nought, so a
second call would find `flown` already set and move nothing.

**The picture is the 3D.** The engine's harness leaves the overlay empty, so
the distance and the line of help are not in shot and a reference does not
change when either does.

**It is the engine's size**, 320 by 240, said once in `picture::SIZE` and
read here.

## Acceptance criteria

- The tunnel from inside matches its
  picture. — `slider_game::tests::the_tunnel_from_inside_looks_right`

### Verified by hand

- **The picture itself.** The checker running away to the vanishing point,
  the tunnel bending to the right, the gold ring hanging in the middle
  distance and the red tiles scattered through it: the same view the
  committed `media/screenshot.png` shows, without the window or the
  readout.
- **A control.** `FLOWN_FOR` cut from 12.3 to 11.8, half a second of
  flying, and the test failed on 66.8% of its pixels, which is most of the
  frame because the whole tunnel moves past the camera. Put back and
  compared.

## Out of scope

**The readout.** It is 2D and the engine's harness draws none of it.

**Holding this against the engine's tunnel example.** `./check-tunnel`
already holds the two to the same twenty numbers, which is the thing a
picture cannot do across two repos.
