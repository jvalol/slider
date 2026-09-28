//! The thing you steer: where it sits in the cross section, how fast it is
//! going, and what the wall does to it. See `specs/0002-the-ship.md`.

use crate::tunnel;
use glam::Vec2;

/// How fast the run starts and ends up, in metres a second.
pub const START_SPEED: f32 = 22.0;
pub const TOP_SPEED: f32 = 70.0;
/// The floor a scrape can push you down to.
pub const SCRAPED_SPEED: f32 = START_SPEED * 0.6;

/// How hard steering pushes, and how hard the ship is pulled back to still.
pub const STEER: f32 = 46.0;
pub const DRAG: f32 = 2.6;
/// Radians of steering per pixel of mouse.
pub const SENSITIVITY: f32 = 0.09;

/// How much speed a scrape costs per second, and how much a missed ring costs
/// outright.
pub const SCRAPE_COST: f32 = 26.0;
pub const MISS_COST: f32 = 12.0;
/// How much of the sideways drift a scrape kills.
pub const SCRAPE_GRIP: f32 = 0.35;

/// How quickly speed climbs toward what the distance asks for, and falls back.
const GAINING: f32 = 5.0;
const LOSING: f32 = 14.0;

#[derive(Debug, Copy, Clone)]
pub struct Ship {
    /// How far down the tunnel, from 0 at the mouth to 1 at the far end.
    pub along: f32,
    /// Where in the cross section, in radii, and how fast that is changing.
    pub offset: Vec2,
    pub drift: Vec2,
    pub speed: f32,
    /// Whether the wall is being touched right now.
    pub scraping: bool,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            along: 0.0,
            offset: Vec2::ZERO,
            drift: Vec2::ZERO,
            speed: START_SPEED,
            scraping: false,
        }
    }

    /// One frame. `steer` is the mouse movement for this frame, already in
    /// steering units. Says how far down the tunnel the ship was before, so a
    /// caller can tell which rings it has just passed.
    pub fn update(&mut self, steer: Vec2, dt: f32) -> f32 {
        let was = self.along;

        // steering pushes and the middle pulls, so letting go settles rather
        // than leaving the ship pinned wherever the mouse stopped
        self.drift += steer * STEER * dt;
        self.drift -= self.drift * DRAG * dt;
        self.offset += self.drift * dt;

        let out = self.offset.length();
        self.scraping = out > tunnel::CLEARANCE;
        if self.scraping {
            // slide along the wall rather than stopping dead against it
            self.offset = self.offset / out * tunnel::CLEARANCE;
            self.drift *= SCRAPE_GRIP;
            self.speed = (self.speed - SCRAPE_COST * dt).max(SCRAPED_SPEED);
        }

        let wanted = START_SPEED + (TOP_SPEED - START_SPEED) * self.along;
        self.speed += (wanted - self.speed).clamp(-LOSING * dt, GAINING * dt);
        self.along = (self.along + self.speed * dt / tunnel::LENGTH).min(1.0);

        was
    }

    /// What a missed ring costs.
    pub fn miss(&mut self) {
        self.speed = (self.speed - MISS_COST).max(SCRAPED_SPEED);
    }

    /// Where the ship is, out in the world.
    pub fn position(&self) -> glam::Vec3 {
        tunnel::at(self.along, self.offset)
    }

    pub fn is_out(&self) -> bool {
        self.along >= 1.0
    }
}

impl Default for Ship {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::vec2;

    /// Runs the ship for a while with a steady hand on the mouse.
    fn fly(ship: &mut Ship, steer: Vec2, seconds: f32) {
        let dt = 1.0 / 60.0;
        let mut elapsed = 0.0;

        while elapsed < seconds {
            ship.update(steer, dt);
            elapsed += dt;
        }
    }

    #[test]
    fn a_new_ship_starts_at_the_mouth_in_the_middle() {
        let ship = Ship::new();

        assert_eq!(ship.along, 0.0);
        assert_eq!(ship.offset, Vec2::ZERO);
        assert_eq!(ship.speed, START_SPEED);
        assert!(!ship.scraping);
    }

    #[test]
    fn it_flies_itself_down_the_tunnel() {
        let mut ship = Ship::new();
        fly(&mut ship, Vec2::ZERO, 2.0);

        assert!(ship.along > 0.0, "it did not move");
        assert_eq!(ship.offset, Vec2::ZERO, "it wandered with no hand on it");
    }

    #[test]
    fn steering_moves_it_that_way() {
        let mut ship = Ship::new();
        fly(&mut ship, vec2(0.01, 0.0), 0.5);

        assert!(ship.offset.x > 0.0, "it went {:?}", ship.offset);
        assert!(ship.offset.y.abs() < 1e-6, "it drifted off the axis");
    }

    #[test]
    fn letting_go_settles_rather_than_leaving_it_sliding() {
        let mut ship = Ship::new();
        fly(&mut ship, vec2(0.02, 0.0), 0.4);
        let moving = ship.drift.length();

        fly(&mut ship, Vec2::ZERO, 3.0);

        assert!(moving > 0.0, "the test never got it moving");
        assert!(
            ship.drift.length() < moving * 0.1,
            "it was still sliding at {:?}",
            ship.drift
        );
    }

    #[test]
    fn the_wall_holds_it_in() {
        let mut ship = Ship::new();
        fly(&mut ship, vec2(0.05, 0.05), 6.0);

        assert!(ship.scraping, "it never reached the wall");
        assert!(
            ship.offset.length() <= tunnel::CLEARANCE + 1e-5,
            "it went through the wall to {:.2}",
            ship.offset.length()
        );
    }

    #[test]
    fn scraping_costs_speed_and_the_floor_holds() {
        let mut ship = Ship::new();
        fly(&mut ship, Vec2::ZERO, 8.0);
        let clean = ship.speed;

        let mut scraped = Ship::new();
        fly(&mut scraped, vec2(0.05, 0.0), 8.0);

        assert!(scraped.scraping);
        assert!(scraped.speed < clean, "scraping was free");
        assert!(
            scraped.speed >= SCRAPED_SPEED - 1e-4,
            "it was ground to a halt"
        );
    }

    #[test]
    fn it_speeds_up_the_further_it_gets() {
        let mut ship = Ship::new();
        fly(&mut ship, Vec2::ZERO, 5.0);
        let early = ship.speed;

        ship.along = 0.9;
        fly(&mut ship, Vec2::ZERO, 5.0);

        assert!(ship.speed > early, "it never picked up");
        assert!(ship.speed <= TOP_SPEED + 1e-3);
    }

    #[test]
    fn a_missed_ring_costs_speed_but_never_stops_it() {
        let mut ship = Ship::new();

        for _ in 0..20 {
            ship.miss();
        }

        assert!(ship.speed >= SCRAPED_SPEED - 1e-4, "it stopped dead");
    }

    #[test]
    fn it_comes_out_the_far_end_and_stops_there() {
        let mut ship = Ship::new();
        fly(&mut ship, Vec2::ZERO, 60.0);

        assert!(ship.is_out());
        assert_eq!(ship.along, 1.0, "it flew out past the end of the tunnel");
    }

    #[test]
    fn a_frame_says_where_it_started() {
        let mut ship = Ship::new();
        ship.along = 0.4;
        let was = ship.update(Vec2::ZERO, 1.0 / 60.0);

        assert!((was - 0.4).abs() < 1e-6);
        assert!(ship.along > was, "it did not advance");
    }

    #[test]
    fn where_it_is_in_the_world_follows_the_tunnel() {
        let mut ship = Ship::new();
        ship.along = 0.25;

        assert!((ship.position() - tunnel::spine(0.25)).length() < 1e-5);

        ship.offset = vec2(0.5, 0.0);
        assert!((ship.position() - tunnel::spine(0.25)).length() > 1.0);
    }
}
