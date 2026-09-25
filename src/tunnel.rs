//! The tunnel: where its middle runs, which way is up inside it, and where its
//! wall is. See `specs/0001-the-tunnel.md`.

use glam::{vec3, Vec2, Vec3};
#[cfg(test)]
use glam::Vec3Swizzles;

/// How long the tunnel is, end to end.
pub const LENGTH: f32 = 900.0;
pub const RADIUS: f32 = 4.0;
/// How finely the wall is cut, along the tunnel and around it.
pub const RINGS: u32 = 900;
pub const AROUND: u32 = 28;

/// How far the middle wanders sideways and up, and how many times over the
/// length. Two rates that do not divide into each other, so the tunnel never
/// settles into a shape you can learn.
pub const WANDER_X: f32 = 11.0;
pub const WANDER_Y: f32 = 7.0;
pub const TURNS_X: f32 = 6.0;
pub const TURNS_Y: f32 = 9.0;

/// How much of the radius the ship may use before it is scraping the wall.
pub const CLEARANCE: f32 = 0.82;

/// How many times the checker repeats along the tunnel and around it. `surface`
/// hands back the parameters themselves as texture coordinates, which over this
/// length would stretch one square the whole way.
pub const TILES_ALONG: f32 = 150.0;
pub const TILES_AROUND: f32 = 8.0;

/// The middle of the tunnel at `along`, which runs 0 at the mouth to 1 at the
/// far end.
pub fn spine(along: f32) -> Vec3 {
    vec3(
        (along * TURNS_X * std::f32::consts::TAU).sin() * WANDER_X,
        (along * TURNS_Y * std::f32::consts::TAU).sin() * WANDER_Y,
        -along * LENGTH,
    )
}

/// Which way the tunnel runs at `along`.
pub fn heading(along: f32) -> Vec3 {
    let step = 0.5 / RINGS as f32;

    (spine((along + step).min(1.0)) - spine((along - step).max(0.0))).normalize()
}

/// The way round the tunnel at `along`: an across and an up, both square to the
/// heading. Built from world up each time rather than carried along the tunnel,
/// which holds while the tunnel never points straight up.
pub fn frame(along: f32) -> (Vec3, Vec3) {
    let ahead = heading(along);
    let across = ahead.cross(Vec3::Y).normalize();

    (across, across.cross(ahead))
}

/// A point in the tunnel's cross section, out in the world. `offset` is in
/// radii, so a length of one is the wall.
pub fn at(along: f32, offset: Vec2) -> Vec3 {
    let (across, up) = frame(along);

    spine(along) + (across * offset.x + up * offset.y) * RADIUS
}

/// A point on the wall itself. `around` goes once round.
pub fn wall(along: f32, around: f32) -> Vec3 {
    let (sin, cos) = (around * std::f32::consts::TAU).sin_cos();

    at(along, Vec2::new(cos, sin))
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::vec2;

    #[test]
    fn the_tunnel_runs_from_the_mouth_to_the_far_end() {
        assert!((spine(0.0).z).abs() < 1e-4);
        assert!((spine(1.0).z + LENGTH).abs() < 1e-3);
        // and it wanders on the way rather than running straight
        assert!(spine(0.3).xy().length() > 1.0);
    }

    #[test]
    fn the_middle_stays_inside_what_it_wanders() {
        for step in 0..=200 {
            let along = step as f32 / 200.0;
            let middle = spine(along);

            assert!(middle.x.abs() <= WANDER_X + 1e-4);
            assert!(middle.y.abs() <= WANDER_Y + 1e-4);
        }
    }

    #[test]
    fn the_heading_always_points_down_the_tunnel() {
        for step in 0..=200 {
            let along = step as f32 / 200.0;
            let ahead = heading(along);

            assert!((ahead.length() - 1.0).abs() < 1e-4, "at {}", along);
            assert!(ahead.z < 0.0, "at {} it heads back up the tunnel", along);
        }
    }

    #[test]
    fn the_frame_is_square_to_the_heading_and_to_itself() {
        for step in 0..=200 {
            let along = step as f32 / 200.0;
            let ahead = heading(along);
            let (across, up) = frame(along);

            for (name, vector) in [("across", across), ("up", up)] {
                assert!((vector.length() - 1.0).abs() < 1e-4, "{} at {}", name, along);
                assert!(vector.dot(ahead).abs() < 1e-4, "{} leans down the tunnel", name);
            }
            assert!(across.dot(up).abs() < 1e-4, "the frame is not square at {}", along);
        }
    }

    #[test]
    fn the_wall_is_one_radius_out_all_the_way_round() {
        for step in 0..20 {
            let along = step as f32 / 20.0;

            for turn in 0..16 {
                let around = turn as f32 / 16.0;
                let out = wall(along, around) - spine(along);

                assert!((out.length() - RADIUS).abs() < 1e-3, "at {} {}", along, around);
                // and square to the tunnel, so the wall is a tube rather than
                // a sheared one
                assert!(out.dot(heading(along)).abs() < 1e-3);
            }
        }
    }

    #[test]
    fn the_wall_closes_on_itself() {
        for step in 0..10 {
            let along = step as f32 / 10.0;

            assert!((wall(along, 0.0) - wall(along, 1.0)).length() < 1e-4);
        }
    }

    #[test]
    fn the_middle_of_the_cross_section_is_the_middle_of_the_tunnel() {
        for step in 0..10 {
            let along = step as f32 / 10.0;

            assert!((at(along, Vec2::ZERO) - spine(along)).length() < 1e-5);
        }
    }

    #[test]
    fn an_offset_of_one_reaches_the_wall() {
        let along = 0.37;
        let out = at(along, vec2(1.0, 0.0)) - spine(along);

        assert!((out.length() - RADIUS).abs() < 1e-4);
    }
}
