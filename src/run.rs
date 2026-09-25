//! A run down the tunnel: which rings have been judged, how many were taken,
//! and when it is over. See `specs/0004-the-run.md`.

use crate::ring::{self, COUNT};
use crate::ship::Ship;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Phase {
    Flying,
    Finished,
}

#[derive(Debug, Copy, Clone)]
pub struct Run {
    pub phase: Phase,
    pub taken: usize,
    pub missed: usize,
    /// The next ring still ahead, which is the one to aim at.
    pub next: usize,
    pub time: f32,
}

impl Run {
    pub fn new() -> Self {
        Self {
            phase: Phase::Flying,
            taken: 0,
            missed: 0,
            next: 0,
            time: 0.0,
        }
    }

    pub fn is_flying(&self) -> bool {
        self.phase == Phase::Flying
    }

    /// Judges every ring the ship has just gone past, and ends the run at the
    /// far end. `was` is where the ship was before this frame.
    ///
    /// A ring is judged once, on the frame the ship passes it, by where the
    /// ship is then. Passing two in one frame is possible at speed and both
    /// are judged, so a fast run cannot skip one by being too quick.
    pub fn tick(&mut self, dt: f32, was: f32, ship: &mut Ship) {
        if !self.is_flying() {
            return;
        }

        self.time += dt;

        while self.next < COUNT {
            let ring = ring::ring(self.next);
            if ring.along > ship.along || ring.along < was {
                break;
            }

            if ring.takes(ship.offset) {
                self.taken += 1;
            } else {
                self.missed += 1;
                ship.miss();
            }
            self.next += 1;
        }

        if ship.is_out() {
            self.phase = Phase::Finished;
        }
    }

    /// The rings still to come, nearest first. The first of them is the one
    /// lit up, per spec 0003.
    pub fn ahead(&self) -> impl Iterator<Item = (usize, ring::Ring)> {
        (self.next..COUNT).map(|index| (index, ring::ring(index)))
    }
}

impl Default for Run {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec2;

    /// Flies a whole run with a hand that holds the ship wherever `hold` says.
    fn run_through(hold: impl Fn(usize) -> Vec2) -> (Run, Ship) {
        let mut run = Run::new();
        let mut ship = Ship::new();
        let dt = 1.0 / 60.0;
        let mut frames = 0;

        while run.is_flying() && frames < 60 * 120 {
            // put the ship where the hand wants it before the frame is judged
            ship.offset = hold(run.next);
            let was = ship.update(Vec2::ZERO, dt);
            run.tick(dt, was, &mut ship);
            frames += 1;
        }

        (run, ship)
    }

    #[test]
    fn a_run_that_takes_every_ring_takes_every_ring() {
        let (run, _) = run_through(|next| {
            if next < COUNT {
                ring::ring(next).centre
            } else {
                Vec2::ZERO
            }
        });

        assert_eq!(run.phase, Phase::Finished);
        assert_eq!(run.taken, COUNT);
        assert_eq!(run.missed, 0);
    }

    #[test]
    fn a_run_that_never_steers_misses_most_of_them() {
        let (run, _) = run_through(|_| Vec2::ZERO);

        assert_eq!(run.taken + run.missed, COUNT, "some ring went unjudged");
        assert!(run.missed > run.taken, "the middle took {} of them", run.taken);
    }

    #[test]
    fn every_ring_is_judged_exactly_once() {
        let (run, _) = run_through(|_| Vec2::ZERO);

        assert_eq!(run.next, COUNT);
        assert_eq!(run.taken + run.missed, COUNT);
    }

    #[test]
    fn a_ring_is_not_judged_before_the_ship_reaches_it() {
        let mut run = Run::new();
        let mut ship = Ship::new();
        ship.along = 0.0;

        let was = ship.update(Vec2::ZERO, 1.0 / 60.0);
        run.tick(1.0 / 60.0, was, &mut ship);

        assert_eq!(run.next, 0, "a ring was judged at the mouth");
    }

    #[test]
    fn two_rings_passed_in_one_frame_are_both_judged() {
        // a long frame, the kind a hitch produces, must not let a ring slip by
        let mut run = Run::new();
        let mut ship = Ship::new();
        let third = ring::ring(2).along;

        ship.along = third + 0.001;
        run.tick(1.0, 0.0, &mut ship);

        assert_eq!(run.next, 3, "only {} rings were judged", run.next);
        assert_eq!(run.taken + run.missed, 3);
    }

    #[test]
    fn missing_a_ring_costs_the_ship_speed() {
        let mut run = Run::new();
        let mut ship = Ship::new();
        ship.offset = Vec2::ZERO;
        ship.along = ring::ring(0).along + 0.001;
        // somewhere the first ring is not
        let before = ship.speed;

        run.tick(1.0 / 60.0, 0.0, &mut ship);

        if run.missed > 0 {
            assert!(ship.speed < before, "a miss was free");
        }
    }

    #[test]
    fn the_run_ends_at_the_far_end() {
        let (run, ship) = run_through(|_| Vec2::ZERO);

        assert_eq!(run.phase, Phase::Finished);
        assert!(ship.is_out());
        assert!(run.time > 0.0);
    }

    #[test]
    fn a_finished_run_stops_its_clock() {
        let (mut run, mut ship) = run_through(|_| Vec2::ZERO);
        let stopped = run.time;

        for _ in 0..600 {
            run.tick(1.0 / 60.0, ship.along, &mut ship);
        }

        assert_eq!(run.time, stopped);
    }

    #[test]
    fn what_is_ahead_shrinks_as_rings_go_by() {
        let mut run = Run::new();
        assert_eq!(run.ahead().count(), COUNT);

        run.next = 5;
        assert_eq!(run.ahead().count(), COUNT - 5);
        assert_eq!(run.ahead().next().unwrap().0, 5);

        run.next = COUNT;
        assert_eq!(run.ahead().count(), 0, "it still says to aim somewhere");
    }
}
