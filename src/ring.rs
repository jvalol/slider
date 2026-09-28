//! The rings you fly through. Without them there is nowhere in the cross
//! section worth being, and steering has nothing to answer.
//! See `specs/0003-rings.md`.

use glam::{vec2, Vec2};

/// How many rings are strung down the tunnel.
pub const COUNT: usize = 14;
/// How wide the hole is, in radii.
pub const HOLE: f32 = 0.42;
/// How thick the ring itself is, in radii.
pub const RIM: f32 = 0.055;
/// How far from the middle every ring sits. Two things pin this. It has to be
/// more than `HOLE`, or a ship flying straight down the middle would take every
/// ring without steering once. And it plus the far side of the hole has to
/// clear the wall.
pub const WANDER: f32 = 0.46;
/// How far around the circle each ring is from the last. The golden angle,
/// which is the one that never settles into a repeating pattern of directions.
const TURN: f32 = 2.399_963;

#[derive(Debug, Copy, Clone)]
pub struct Ring {
    /// How far down the tunnel, from 0 to 1.
    pub along: f32,
    /// Where in the cross section, in radii.
    pub centre: Vec2,
}

impl Ring {
    /// Whether a ship at `offset` is inside the hole.
    pub fn takes(&self, offset: Vec2) -> bool {
        (offset - self.centre).length() < HOLE
    }
}

/// The `index`th ring. Evenly spaced along the tunnel, and every one the same
/// distance off its middle, differing only in which way.
///
/// The direction steps by the golden angle, so the rings never fall into a
/// rhythm and every quarter of the tunnel gets used. Keeping the distance fixed
/// rather than varying it is what makes each ring the same size of problem, and
/// what keeps the maths for clearing the wall to one line.
pub fn ring(index: usize) -> Ring {
    let along = (index + 1) as f32 / (COUNT + 1) as f32;
    let (sin, cos) = (index as f32 * TURN).sin_cos();

    Ring {
        along,
        centre: vec2(cos, sin) * WANDER,
    }
}

/// A torus about the y axis, in radii: the ring before it is turned to face
/// down the tunnel.
pub fn torus(around: f32, through: f32) -> glam::Vec3 {
    let (sin_a, cos_a) = (around * std::f32::consts::TAU).sin_cos();
    let (sin_t, cos_t) = (through * std::f32::consts::TAU).sin_cos();
    let out = HOLE + RIM * cos_t;

    glam::vec3(out * cos_a, RIM * sin_t, out * sin_a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tunnel;
    use glam::Vec3Swizzles;

    #[test]
    fn the_rings_march_down_the_tunnel_in_order() {
        let mut last = 0.0;

        for index in 0..COUNT {
            let ring = ring(index);

            assert!(ring.along > last, "ring {} doubled back", index);
            assert!(ring.along < 1.0, "ring {} is past the far end", index);
            last = ring.along;
        }
    }

    #[test]
    fn no_ring_is_at_the_mouth_or_the_far_end() {
        // one at either end would be unreachable: no time to steer to the
        // first, and the run is over at the last
        assert!(ring(0).along > 0.02);
        assert!(ring(COUNT - 1).along < 0.98);
    }

    #[test]
    fn every_ring_fits_inside_the_tunnel() {
        for index in 0..COUNT {
            let ring = ring(index);
            let furthest = ring.centre.length() + HOLE + RIM;

            assert!(
                furthest <= 1.0,
                "ring {} reaches {:.2} radii, which is through the wall",
                index,
                furthest
            );
        }
    }

    #[test]
    fn a_ring_can_be_reached_without_scraping() {
        // the middle of every hole has to sit inside what the ship is allowed
        // to use, or the ring could only be taken by grinding the wall
        for index in 0..COUNT {
            assert!(
                ring(index).centre.length() < tunnel::CLEARANCE,
                "ring {} sits outside the clearance",
                index
            );
        }
    }

    #[test]
    fn the_rings_are_not_all_in_one_place() {
        let mut quadrants = [0u32; 4];

        for index in 0..COUNT {
            let centre = ring(index).centre;
            let angle = centre.y.atan2(centre.x) + std::f32::consts::PI;
            quadrants[((angle / std::f32::consts::FRAC_PI_2) as usize).min(3)] += 1;
        }

        for (index, count) in quadrants.iter().enumerate() {
            assert!(*count > 0, "nothing ever appears in quadrant {}", index);
        }
    }

    #[test]
    fn the_hole_is_taken_from_inside_and_missed_from_outside() {
        let ring = ring(3);

        assert!(ring.takes(ring.centre));
        assert!(ring.takes(ring.centre + Vec2::X * HOLE * 0.9));
        assert!(!ring.takes(ring.centre + Vec2::X * HOLE * 1.1));
    }

    #[test]
    fn flying_down_the_middle_takes_nothing() {
        // if the middle took any of them, that ring would need no steering,
        // and the one thing this game asks of you is steering
        let taken = (0..COUNT)
            .filter(|index| ring(*index).takes(Vec2::ZERO))
            .count();

        assert_eq!(taken, 0, "{} of {} need no steering", taken, COUNT);
    }

    #[test]
    fn every_ring_is_the_same_size_of_problem() {
        for index in 0..COUNT {
            assert!(
                (ring(index).centre.length() - WANDER).abs() < 1e-5,
                "ring {} sits {:.3} out rather than {:.3}",
                index,
                ring(index).centre.length(),
                WANDER
            );
        }
    }

    #[test]
    fn the_torus_is_a_ring_around_the_hole() {
        for step in 0..16 {
            let around = step as f32 / 16.0;
            // the near and far side of the rim, across the tube of the torus
            let outer = torus(around, 0.0);
            let inner = torus(around, 0.5);

            assert!((outer.xz().length() - (HOLE + RIM)).abs() < 1e-5);
            assert!((inner.xz().length() - (HOLE - RIM)).abs() < 1e-5);
            // it lies in the plane the ship flies through
            assert!(outer.y.abs() < 1e-5);
        }
    }
}
