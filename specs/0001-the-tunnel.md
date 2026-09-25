# 0001 The tunnel

**Status:** implemented
**Date:** 2026-09-25

## Goal

Somewhere to fly: a tube that wanders, seen only from the inside.

## Behavior

The tunnel is nine hundred metres long and four across. Its middle wanders
sideways and up as it goes, on two rates that do not divide into each other, so
it never settles into a shape you can learn and fly from memory.

**Along** runs from 0 at the mouth to 1 at the far end. At any point along it
the tunnel has a heading, which is the way it runs, and a frame, an across and
an up square to that heading and to each other. Everything in the cross section
is given in **radii**, so an offset of length one is the wall and zero is the
middle.

The frame is rebuilt from world up at each point rather than carried along the
tunnel. That is enough while the tunnel never points straight up, and it is
simpler than transporting a frame; if the tunnel ever did point straight up,
the across would have nothing to be square to.

**The wall is drawn from both sides**, because the side you are on is the
inside. Without that the tunnel would have no walls at all, and this is the case
the engine's two-sided geometry exists for.

**The checker is built, not loaded.** No image file travels with this game. It
is four squares, which the wall then tiles 150 times along and 8 times around.
The engine hands back the surface parameters themselves as texture coordinates,
and over nine hundred metres that would stretch one square the whole way, so the
tiling happens after the mesh is built. The tiling is what makes speed readable
and what gives the mip chain anything to do.

## Acceptance criteria

- The tunnel runs from the mouth to the far end, and wanders on the way. — `tunnel::tests::the_tunnel_runs_from_the_mouth_to_the_far_end`
- Its middle stays inside what it is allowed to wander. — `tunnel::tests::the_middle_stays_inside_what_it_wanders`
- The heading is a unit vector that always points down the tunnel. — `tunnel::tests::the_heading_always_points_down_the_tunnel`
- The frame is square to the heading and to itself, everywhere. — `tunnel::tests::the_frame_is_square_to_the_heading_and_to_itself`
- The wall is one radius out all the way round, and square to the tunnel. — `tunnel::tests::the_wall_is_one_radius_out_all_the_way_round`
- The wall closes on itself rather than leaving a slit. — `tunnel::tests::the_wall_closes_on_itself`
- The middle of the cross section is the middle of the tunnel. — `tunnel::tests::the_middle_of_the_cross_section_is_the_middle_of_the_tunnel`
- An offset of one reaches the wall. — `tunnel::tests::an_offset_of_one_reaches_the_wall`
- The checker is the size it says. — `checker::tests::the_image_is_the_size_it_says`
- It is a checker, not stripes. — `checker::tests::it_is_a_checker_not_stripes`
- There are enough squares to read as speed. — `checker::tests::there_are_enough_squares_to_read_as_speed`
- Exactly one square is marked. — `checker::tests::exactly_one_square_is_marked`
- Each square is one flat color. — `checker::tests::a_square_is_all_one_color`
- It tiles without a seam. — `checker::tests::it_tiles_without_a_seam`
- The two colors are far enough apart to tell apart. — `checker::tests::the_two_colors_are_far_enough_apart_to_see`
- Every pixel is written, and the mip chain runs down to one. — `checker::tests::every_pixel_is_written_and_the_mips_follow`

### Verified by hand

- The tunnel is solid the whole way, with no holes where the wall turns away.
- The checker holds to the vanishing point rather than breaking into shimmer.

## Out of scope

Branches, junctions, a tunnel that changes width, and a tunnel that points far
enough up to break the frame.
