//! What the hand on the mouse did this frame. See `specs/0002-the-ship.md`.

use blitzkit::keyboard::{KeyboardInput, KeyboardKey, KeyboardKeyState};
use blitzkit::mouse::{MouseButton, MouseInput};
use glam::{vec2, Vec2};

#[derive(Debug, Default)]
pub struct Input {
    /// Steering for this frame, already scaled from pixels.
    pub steer: Vec2,
    pub restart: bool,
    pub toggle_cursor: bool,
    /// The left button went down while the cursor was loose, which asks for the
    /// cursor rather than doing anything in the tunnel.
    pub grab_cursor: bool,
    pub quitting: bool,
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn keyboard(&mut self, input: KeyboardInput) {
        let held = input.state == KeyboardKeyState::Pressed;

        match input.key {
            KeyboardKey::Escape => self.quitting = held,
            KeyboardKey::R if held => self.restart = true,
            KeyboardKey::Space if held => self.toggle_cursor = true,
            _ => (),
        }
    }

    pub fn mouse(&mut self, input: MouseInput, cursor_locked: bool) {
        if input.button == MouseButton::Left && input.is_pressed() && !cursor_locked {
            self.grab_cursor = true;
        }
    }

    /// Screen down is positive and down in the tunnel is negative, so the
    /// second of these is turned over on the way in.
    pub fn mouse_motion(&mut self, delta: Vec2, cursor_locked: bool) {
        if cursor_locked {
            self.steer += vec2(delta.x, -delta.y) * crate::ship::SENSITIVITY;
        }
    }

    pub fn clear_frame(&mut self) {
        self.steer = Vec2::ZERO;
        self.restart = false;
        self.toggle_cursor = false;
        self.grab_cursor = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blitzkit::mouse::ButtonState;

    fn press(key: KeyboardKey) -> KeyboardInput {
        KeyboardInput {
            key,
            state: KeyboardKeyState::Pressed,
            repeat: false,
        }
    }

    #[test]
    fn the_ship_only_steers_while_the_cursor_is_held() {
        let mut input = Input::new();

        input.mouse_motion(vec2(10.0, 0.0), false);
        assert_eq!(input.steer, Vec2::ZERO, "it steered with the cursor loose");

        input.mouse_motion(vec2(10.0, 0.0), true);
        assert!(input.steer.x > 0.0);
    }

    #[test]
    fn down_on_the_screen_is_down_in_the_tunnel() {
        let mut input = Input::new();
        input.mouse_motion(vec2(0.0, 10.0), true);

        assert!(input.steer.y < 0.0, "pushing the mouse down steered up");
    }

    #[test]
    fn movement_within_a_frame_adds_up() {
        let mut input = Input::new();
        input.mouse_motion(vec2(3.0, 0.0), true);
        input.mouse_motion(vec2(2.0, 0.0), true);

        assert!((input.steer.x - 5.0 * crate::ship::SENSITIVITY).abs() < 1e-6);
    }

    #[test]
    fn a_click_with_the_cursor_loose_asks_for_the_cursor() {
        let mut input = Input::new();
        input.mouse(
            MouseInput::new(MouseButton::Left, ButtonState::Pressed),
            false,
        );

        assert!(input.grab_cursor);
    }

    #[test]
    fn a_click_with_the_cursor_held_does_nothing() {
        let mut input = Input::new();
        input.mouse(
            MouseInput::new(MouseButton::Left, ButtonState::Pressed),
            true,
        );

        assert!(!input.grab_cursor);
    }

    #[test]
    fn a_frame_starts_clean() {
        let mut input = Input::new();
        input.mouse_motion(vec2(9.0, 9.0), true);
        input.keyboard(press(KeyboardKey::R));
        input.keyboard(press(KeyboardKey::Space));

        input.clear_frame();

        assert_eq!(input.steer, Vec2::ZERO);
        assert!(!input.restart);
        assert!(!input.toggle_cursor);
    }

    #[test]
    fn escape_outlives_the_frame_it_was_pressed_in() {
        let mut input = Input::new();
        input.keyboard(press(KeyboardKey::Escape));
        input.clear_frame();

        assert!(input.quitting, "the window would never close");
    }
}
