use std::time::{Duration, Instant};

use show_image::{make_window_full, KeyCode, Window, WindowOptions};
use ultraviolet::{Bivec3, Rotor3};

use crate::render::renderer::Renderer;

const WAIT_KEY_MS: u64 = 1;
const RENDER_TIME_MS: u64 = 500;
const ROTATION: f32 = std::f32::consts::FRAC_PI_8; // 22.5°

pub struct RenderWindow<T> {
    window: Window,
    renderer: T,
    should_exit: bool,
    should_update_render: bool,
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl<T: Renderer> RenderWindow<T> {
    pub fn new(name: String, mut renderer: T) -> Result<Self, String> {
        let camera = renderer.get_camera();
        let options = WindowOptions::default()
            .set_name(name)
            .set_size([camera.width, camera.height]);

        Ok(Self {
            window: make_window_full(options)?,
            renderer,
            should_exit: false,
            should_update_render: false,
        })
    }

    pub fn start_rendering(&mut self) {
        self.start_rendering_dt(Duration::from_millis(RENDER_TIME_MS));
    }

    pub fn start_rendering_dt(&mut self, render_time: Duration) {
        let wait_key = Duration::from_millis(WAIT_KEY_MS);

        loop {
            println!("# Entering render loop");
            self.render_loop(wait_key, render_time);
            if self.should_exit {
                return;
            }

            println!("# Entering waiting loop");
            self.waiting_loop(wait_key);
            if self.should_exit {
                return;
            }
        }
    }

    fn render_loop(&mut self, wait_key: Duration, render_time: Duration) {
        let mut done = false;
        while let Ok(event) = self.window.wait_key(wait_key) {
            if let Some(event) = event {
                self.handle_input(event.key);
            }

            if self.should_exit {
                return;
            }

            if self.should_update_render {
                self.should_update_render = false;
                self.renderer.reset();
            }

            let now = Instant::now();
            while now.elapsed() < render_time {
                if self.renderer.is_done() {
                    done = true;
                    break;
                }

                self.renderer.render_pass()
            }

            let image = self.renderer.get_image();
            self.window
                .set_image(image, "Rendering")
                .expect("Unable to update image in window");

            if done {
                return;
            }
        }
    }

    fn waiting_loop(&mut self, wait_key: Duration) {
        while let Ok(event) = self.window.wait_key(wait_key) {
            if self.should_exit || self.should_update_render {
                return;
            }

            if let Some(event) = event {
                self.handle_input(event.key);
            }
        }
    }

    fn handle_input(&mut self, input: KeyCode) {
        match input {
            KeyCode::Escape => self.should_exit = true,
            KeyCode::Backspace => self.renderer.reset(),
            KeyCode::ArrowUp => self.rotate_camera(Direction::UP),
            KeyCode::ArrowDown => self.rotate_camera(Direction::DOWN),
            KeyCode::ArrowLeft => self.rotate_camera(Direction::LEFT),
            KeyCode::ArrowRight => self.rotate_camera(Direction::RIGHT),
            _ => {}
        }
    }

    fn rotate_camera(&mut self, dir: Direction) {
        let camera = self.renderer.get_camera();

        let rotor = match dir {
            Direction::LEFT => {
                Rotor3::from_angle_plane(-ROTATION, Bivec3::from_normalized_axis(camera.up))
            }

            Direction::RIGHT => {
                Rotor3::from_angle_plane(ROTATION, Bivec3::from_normalized_axis(camera.up))
            }

            Direction::UP => {
                Rotor3::from_angle_plane(-ROTATION, Bivec3::from_normalized_axis(camera.right))
            }

            Direction::DOWN => {
                Rotor3::from_angle_plane(ROTATION, Bivec3::from_normalized_axis(camera.right))
            }
        };

        camera.forward.rotate_by(rotor);
        camera.up.rotate_by(rotor);
        camera.right.rotate_by(rotor);

        // important
        camera.reset();
        self.should_update_render = true;
    }
}
