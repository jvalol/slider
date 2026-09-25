# 0002 The ship

**Status:** implemented
**Date:** 2026-09-25

## Goal

Something to steer, that answers the mouse and is held in by the wall.

## Behavior

The ship flies itself down the tunnel. All you do is move it about the cross
section, and its place there is kept in radii like everything else.

**Steering pushes and the middle pulls.** The mouse adds to a sideways drift
rather than setting a position, and that drift is bled off continuously. Letting
go of the mouse settles the ship rather than leaving it pinned wherever it
stopped. That is what makes small corrections possible near a ring.

**The wall holds it in at 0.82 of the radius.** Reaching that is a scrape, not a
crash: the ship is pushed back to the line, most of its sideways drift is
killed, and it loses speed while it stays there. It slides along the wall rather
than stopping dead against it, because stopping dead at 60 metres a second reads
as a bug.

**Speed climbs with distance**, from 22 metres a second at the mouth to 70 at
the far end, and it climbs more slowly than it falls. Scraping the wall and
missing a ring both cost speed, and neither can take it below 60% of the
starting speed. A run can go badly without grinding to a halt.

**The ship stops at the far end** rather than flying out past it.

**The mouse only steers while the cursor is held.** With the cursor loose, a
click takes hold of it rather than steering, which stops the click that returns
to the window from throwing the ship sideways. Space lets it go, and losing the
window's focus lets it go too.

## Acceptance criteria

- A new ship starts at the mouth, in the middle, at the starting speed. — `ship::tests::a_new_ship_starts_at_the_mouth_in_the_middle`
- It flies itself, and does not wander with no hand on it. — `ship::tests::it_flies_itself_down_the_tunnel`
- Steering moves it that way and not the other. — `ship::tests::steering_moves_it_that_way`
- Letting go settles it rather than leaving it sliding. — `ship::tests::letting_go_settles_rather_than_leaving_it_sliding`
- The wall holds it in rather than letting it through. — `ship::tests::the_wall_holds_it_in`
- Scraping costs speed, and the floor under that holds. — `ship::tests::scraping_costs_speed_and_the_floor_holds`
- It speeds up the further it gets, up to the top speed. — `ship::tests::it_speeds_up_the_further_it_gets`
- A missed ring costs speed but never stops it. — `ship::tests::a_missed_ring_costs_speed_but_never_stops_it`
- It comes out the far end and stops there. — `ship::tests::it_comes_out_the_far_end_and_stops_there`
- A frame says where the ship started, so rings can be judged. — `ship::tests::a_frame_says_where_it_started`
- Where it is in the world follows the tunnel. — `ship::tests::where_it_is_in_the_world_follows_the_tunnel`
- The ship only steers while the cursor is held. — `input::tests::the_ship_only_steers_while_the_cursor_is_held`
- Down on the screen is down in the tunnel. — `input::tests::down_on_the_screen_is_down_in_the_tunnel`
- Movement within a frame adds up. — `input::tests::movement_within_a_frame_adds_up`
- A click with the cursor loose catches the cursor. — `input::tests::a_click_with_the_cursor_loose_asks_for_the_cursor`
- A click with the cursor held does nothing. — `input::tests::a_click_with_the_cursor_held_does_nothing`
- A frame starts with nothing held over. — `input::tests::a_frame_starts_clean`
- Escape outlives the frame it was pressed in. — `input::tests::escape_outlives_the_frame_it_was_pressed_in`

### Verified by hand

- The mouse answers straight away, and a small movement makes a small
  correction rather than throwing the ship across the tunnel.
- Scraping the wall feels like a scrape: it costs you, and you keep going.

## Out of scope

Braking, boosting, rolling the ship, and anything the ship collides with other
than the wall and the rings.
