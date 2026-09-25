//! The `Game` impl: the tunnel, the rings, and what gets drawn.

use blitzkit::camera::Camera;
use blitzkit::geometry::Geometry;
use blitzkit::keyboard::KeyboardInput;
use blitzkit::mesh::{MeshData, Transform};
use blitzkit::mouse::MouseInput;
use blitzkit::renderer::render_text::{RenderText, TextRenderer};
use blitzkit::renderer::scene::{MeshId, Scene, TextureId};
use blitzkit::renderer::Renderer;
use blitzkit::sound::SoundSystem;
use blitzkit::{Game, MAX_DELTA_TIME};
use glam::{vec2, vec4, Quat, Vec2, Vec3, Vec4};

use crate::input::Input;
use crate::ring::{self, COUNT};
use crate::run::{Phase, Run};
use crate::ship::Ship;
use crate::tunnel;

const WALL_TINT: Vec4 = Vec4::new(0.62, 0.66, 0.78, 1.0);
const WALL_PLAIN: Vec4 = Vec4::new(0.30, 0.34, 0.42, 1.0);
/// The one to aim at, and the ones behind it.
const NEXT_RING: Vec4 = Vec4::new(1.0, 0.75, 0.25, 1.0);
const LATER_RING: Vec4 = Vec4::new(0.35, 0.55, 0.70, 1.0);

pub struct SliderGame {
    run: Run,
    ship: Ship,
    input: Input,
    cursor_locked: bool,
    want_cursor_locked: bool,
    tube: Option<MeshId>,
    ring: Option<MeshId>,
    checker: Option<TextureId>,
}

impl SliderGame {
    pub fn new() -> Self {
        Self {
            run: Run::new(),
            ship: Ship::new(),
            input: Input::new(),
            cursor_locked: false,
            want_cursor_locked: false,
            tube: None,
            ring: None,
            checker: None,
        }
    }

    fn restart(&mut self) {
        self.run = Run::new();
        self.ship = Ship::new();
    }

    fn draw_text(&self, text_renderer: &mut TextRenderer) {
        let line = |text: String, y: f32, size: f32, color: Vec4| RenderText {
            position: vec2(20.0, y),
            color,
            text,
            size,
            ..Default::default()
        };
        let white = vec4(1.0, 1.0, 1.0, 1.0);

        if self.run.phase == Phase::Finished {
            text_renderer.push_render_text(line(
                format!("{} of {} rings in {:.1}s", self.run.taken, COUNT, self.run.time),
                20.0,
                24.0,
                white,
            ));
            text_renderer.push_render_text(line(
                String::from("r to go again"),
                52.0,
                14.0,
                white,
            ));
            return;
        }

        text_renderer.push_render_text(line(format!("{:.0} m/s", self.ship.speed), 20.0, 24.0, white));
        text_renderer.push_render_text(line(
            format!(
                "{} of {} rings   {:.0} of {:.0} m{}",
                self.run.taken,
                COUNT,
                self.ship.along * tunnel::LENGTH,
                tunnel::LENGTH,
                if self.ship.scraping { "   scraping" } else { "" }
            ),
            52.0,
            14.0,
            if self.ship.scraping {
                vec4(1.0, 0.55, 0.3, 1.0)
            } else {
                white
            },
        ));

        if !self.cursor_locked {
            text_renderer.push_render_text(line(
                String::from("click to take hold of the cursor, then steer with the mouse"),
                76.0,
                14.0,
                vec4(0.7, 0.7, 0.75, 1.0),
            ));
        }
    }
}

impl Default for SliderGame {
    fn default() -> Self {
        Self::new()
    }
}

impl Game for SliderGame {
    fn load(&mut self, renderer: &mut Renderer) {
        // `surface` hands back the parameters themselves as texture
        // coordinates, which over nine hundred metres would stretch one
        // checker square the whole length of the tunnel
        let mut tube = MeshData::surface(tunnel::RINGS, tunnel::AROUND, tunnel::wall);
        for vertex in tube.vertices.iter_mut() {
            vertex.uv = [
                vertex.uv[0] * tunnel::TILES_ALONG,
                vertex.uv[1] * tunnel::TILES_AROUND,
            ];
        }

        // both sides drawn, because the side you are on is the inside
        self.tube = Some(renderer.add_mesh(&tube.two_sided()));
        self.ring = Some(renderer.add_mesh(&MeshData::surface(32, 10, ring::torus)));
        self.checker = Some(renderer.add_texture(&crate::checker::texture()));

        renderer.set_scene_bounds(blitzkit::collision::Aabb::new(
            Vec3::new(
                -tunnel::WANDER_X - tunnel::RADIUS,
                -tunnel::WANDER_Y - tunnel::RADIUS,
                -tunnel::LENGTH,
            ),
            Vec3::new(
                tunnel::WANDER_X + tunnel::RADIUS,
                tunnel::WANDER_Y + tunnel::RADIUS,
                0.0,
            ),
        ));
    }

    fn initialize(
        &mut self,
        _geometry: &mut Geometry,
        _text_renderer: &mut TextRenderer,
        _sound_system: &SoundSystem,
        _window_size: (f32, f32),
    ) {
    }

    fn before_frame(&mut self, renderer: &mut Renderer) {
        if self.want_cursor_locked != renderer.cursor_locked() {
            // the platform may refuse, so believe what comes back
            self.cursor_locked = renderer.set_cursor_locked(self.want_cursor_locked);
            self.want_cursor_locked = self.cursor_locked;
        }
    }

    fn update(
        &mut self,
        dt: f32,
        _geometry: &mut Geometry,
        text_renderer: &mut TextRenderer,
        _sound_system: &SoundSystem,
    ) {
        let dt = dt.min(MAX_DELTA_TIME);

        if self.input.toggle_cursor {
            self.want_cursor_locked = !self.cursor_locked;
        }
        if self.input.grab_cursor {
            self.want_cursor_locked = true;
        }
        if self.input.restart {
            self.restart();
        }

        if self.run.is_flying() {
            let was = self.ship.update(self.input.steer, dt);
            self.run.tick(dt, was, &mut self.ship);
        }

        self.input.clear_frame();

        text_renderer.reset();
        self.draw_text(text_renderer);
    }

    fn draw(&mut self, scene: &mut Scene, camera: &mut Camera) {
        let (Some(tube), Some(ring_mesh)) = (self.tube, self.ring) else {
            return;
        };

        let wall = Transform::at(Vec3::ZERO);
        match self.checker {
            Some(checker) => scene.push_textured(tube, checker, &wall, WALL_TINT, 24.0),
            None => scene.push_colored(tube, &wall, WALL_PLAIN),
        }

        // the rings still to come, turned to face down the tunnel. The nearest
        // is lit, so there is never a question which to aim for.
        for (index, ring) in self.run.ahead() {
            let middle = tunnel::at(ring.along, ring.centre);
            let facing = Quat::from_rotation_arc(Vec3::Y, tunnel::heading(ring.along));

            scene.push_material(
                ring_mesh,
                &Transform::at(middle)
                    .with_rotation(facing)
                    .with_scale(Vec3::splat(tunnel::RADIUS)),
                if index == self.run.next {
                    NEXT_RING
                } else {
                    LATER_RING
                },
                96.0,
            );
        }

        let at = self.ship.position();
        let ahead = tunnel::heading(self.ship.along);
        camera.position = at - ahead * 1.2;
        camera.target = at + ahead * 30.0;
    }

    fn process_keyboard(&mut self, input: KeyboardInput) {
        self.input.keyboard(input);
    }

    fn process_mouse(&mut self, input: MouseInput) {
        self.input.mouse(input, self.cursor_locked);
    }

    fn mouse_motion(&mut self, delta: Vec2) {
        self.input.mouse_motion(delta, self.cursor_locked);
    }

    fn is_quitting(&self) -> bool {
        self.input.quitting
    }

    fn focus_changed(&mut self, focus: bool) {
        if !focus {
            self.want_cursor_locked = false;
        }
    }
}
