# 0003 Rings

**Status:** implemented
**Date:** 2026-09-25

## Goal

Somewhere to steer to. Without it a tunnel is a fly-through and the mouse
answers nothing.

## Behavior

Fourteen rings are strung down the tunnel, evenly spaced. Each is a hole to fly
through, 0.42 radii wide with a rim of 0.055 around it.

**Every ring sits the same distance off the middle**, 0.46 radii, and differs
only in which way. Two things pin that number:

- It has to be **more than the hole is wide**, or the hole would cover the
  middle of the tunnel and that ring would be taken by a ship flying straight.
  Steering is the only thing this game asks of you, and a ring that needs none
  is a ring that is not there.
- It plus the far side of the hole and its rim has to **stay inside the wall**.
  0.46 and 0.475 come to 0.935 of the radius, which leaves the rim clear.

Keeping the distance fixed rather than varying it makes every ring the same size
of problem, and makes clearing the wall one subtraction rather than a guess.

**The direction steps by the golden angle.** That is the step that never falls
into a repeating set of directions, so consecutive rings are never in the same
place and every quarter of the tunnel gets used.

**The nearest ring still ahead is lit**, and the rest are not, so there is never
a question which one to aim for.

**A ring is judged once**, on the frame the ship passes it, by where the ship is
then. Passing two in one frame is possible at speed or after a hitch, and both
are judged, so a fast run cannot skip one by being too quick.

## Acceptance criteria

- The rings march down the tunnel in order. — `ring::tests::the_rings_march_down_the_tunnel_in_order`
- None sits at the mouth or the far end, where it could not be reached. — `ring::tests::no_ring_is_at_the_mouth_or_the_far_end`
- Every ring fits inside the tunnel, rim included. — `ring::tests::every_ring_fits_inside_the_tunnel`
- Every ring can be reached without scraping the wall. — `ring::tests::a_ring_can_be_reached_without_scraping`
- The rings are not all in one quarter of the tunnel. — `ring::tests::the_rings_are_not_all_in_one_place`
- Every ring is the same distance off the middle. — `ring::tests::every_ring_is_the_same_size_of_problem`
- The hole is taken from inside it and missed from outside. — `ring::tests::the_hole_is_taken_from_inside_and_missed_from_outside`
- Flying down the middle takes nothing at all. — `ring::tests::flying_down_the_middle_takes_nothing`
- The ring is a torus around the hole, in the plane the ship flies through. — `ring::tests::the_torus_is_a_ring_around_the_hole`

### Verified by hand

- The lit ring is obvious from far enough away to steer to it.
- No ring is embedded in the wall.

## Out of scope

Rings that move, rings that shrink, rings worth different amounts, and any
obstacle that is not a ring.
