# 0004 The run

**Status:** implemented
**Date:** 2026-09-25

## Goal

A number at the end, and a reason to fly it again.

## Behavior

A run is one flight from the mouth to the far end. It counts the rings **taken**
and the rings **missed**, and the two always add up to fourteen by the time it
ends: every ring is judged exactly once, whatever the frame rate did.

**Missing costs speed**, which is the only punishment. A sloppy run is a slower
run, and a slower run is a longer one, so the clock carries the cost of the
misses without needing a separate penalty.

**The run ends when the ship reaches the far end.** Its clock stops there rather
than running on behind the finished readout.

R starts a fresh run.

## Acceptance criteria

- A run that takes every ring takes every ring. — `run::tests::a_run_that_takes_every_ring_takes_every_ring`
- A run that never steers misses most of them. — `run::tests::a_run_that_never_steers_misses_most_of_them`
- Every ring is judged exactly once. — `run::tests::every_ring_is_judged_exactly_once`
- A ring is not judged before the ship reaches it. — `run::tests::a_ring_is_not_judged_before_the_ship_reaches_it`
- Two rings passed in one frame are both judged. — `run::tests::two_rings_passed_in_one_frame_are_both_judged`
- Missing a ring costs the ship speed. — `run::tests::missing_a_ring_costs_the_ship_speed`
- The run ends at the far end. — `run::tests::the_run_ends_at_the_far_end`
- A finished run stops its clock. — `run::tests::a_finished_run_stops_its_clock`
- What is still ahead shrinks as rings go by. — `run::tests::what_is_ahead_shrinks_as_rings_go_by`

### Verified by hand

- The number at the end is the one you counted while flying.
- R gives a fresh run rather than dropping you back mid tunnel.

## Out of scope

A best time that survives the process, more than one tunnel, and any scoring
beyond rings taken and the clock.
