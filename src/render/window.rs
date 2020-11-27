use std::time::{Duration, Instant};

use show_image::{make_window_full, KeyCode, Window, WindowOptions};
use ultraviolet::{Bivec3, Rotor3};

use crate::configuration::Configuration;
use crate::render::renderer::Renderer;
use image::ImageFormat;

const WAIT_KEY_MS: u64 = 1;
const RENDER_TIME_MS: u64 = 1000 / 4;
const ROTATION: f32 = -std::f32::consts::FRAC_PI_8 / 2.0; // 11.25°

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct RenderWindow {
    window: Window,
    config: Configuration,
    renderer: Renderer,
    should_exit: bool,
    should_reset_image: bool,
}

impl RenderWindow {
    pub fn new(name: String, config: Configuration, renderer: Renderer) -> Result<Self, String> {
        let div = f32::max(config.width as f32 / 900.0, config.height as f32 / 900.0).max(1.0);
        let width = (config.width as f32 / div) as u32;
        let height = (config.height as f32 / div) as u32;

        let options = WindowOptions::default()
            .set_name(name)
            .set_size([width, height])
            .set_resizable(true)
            .set_preserve_aspect_ratio(true);

        Ok(Self {
            window: make_window_full(options)?,
            config,
            renderer,
            should_exit: false,
            should_reset_image: false,
        })
    }

    pub fn start_rendering(&mut self) {
        self.start_rendering_dt(Duration::from_millis(RENDER_TIME_MS));
    }

    pub fn start_rendering_dt(&mut self, render_time: Duration) {
        let wait_key = Duration::from_millis(WAIT_KEY_MS);

        loop {
            if self.config.verbose {
                println!("RenderWindow: Entering render loop");
            }
            self.render_loop(wait_key, render_time);
            if self.should_exit {
                return;
            }

            if self.config.verbose {
                println!("RenderWindow: Entering waiting loop");
            }
            self.waiting_loop(wait_key);
            if self.should_exit {
                return;
            }
        }
    }

    fn render_loop(&mut self, wait_key: Duration, render_time: Duration) {
        let mut iteration = 1;
        while let Ok(event) = self.window.wait_key(wait_key) {
            if let Some(event) = event {
                self.handle_input(event.key);
            }

            if self.should_exit {
                return;
            }

            if self.should_reset_image {
                self.should_reset_image = false;
                self.renderer.reset_progress();
                self.renderer.reset_image();
            }

            let now = Instant::now();
            while now.elapsed() < render_time {
                if self.try_render_pass() {
                    break;
                }
            }

            let image = self.renderer.get_image_u8();
            if let Some(e) = self.window.set_image(image, "Rendering").err() {
                eprintln!("{}\nSkipping this image!", e);
            }

            if self.renderer.is_done() {
                println!("Iteration done: {}", iteration);
                self.renderer.reset_progress();
                if iteration == self.config.passes {
                    return;
                }
                iteration += 1;
            }
        }
    }

    fn waiting_loop(&mut self, wait_key: Duration) {
        while let Ok(event) = self.window.wait_key(wait_key) {
            if self.should_exit || self.should_reset_image {
                return;
            }

            if let Some(event) = event {
                self.handle_input(event.key);
            }
        }
    }

    fn try_render_pass(&mut self) -> bool {
        if self.renderer.is_done() {
            true
        } else {
            self.renderer.render_pass();

            false
        }
    }

    fn handle_input(&mut self, input: KeyCode) {
        match input {
            KeyCode::Escape => self.should_exit = true,
            KeyCode::Backspace => self.should_reset_image = true,
            KeyCode::Enter => {
                println!("Saving to ./rendering ...");
                self.renderer
                    .get_image_u16()
                    .save_with_format("./rendering.png", ImageFormat::Png)
                    .expect("Could not save image");
                println!("Successfully saved");
            }
            KeyCode::ArrowUp => self.rotate_camera(Direction::UP),
            KeyCode::ArrowDown => self.rotate_camera(Direction::DOWN),
            KeyCode::ArrowLeft => self.rotate_camera(Direction::LEFT),
            KeyCode::ArrowRight => self.rotate_camera(Direction::RIGHT),
            _ => {}
        }
    }

    fn rotate_camera(&mut self, dir: Direction) {
        let mut camera = *self.renderer.get_camera();
        let direction = camera.position - camera.center;

        let new_direction = match dir {
            Direction::LEFT => Some(direction.rotated_by(Rotor3::from_rotation_xz(-ROTATION))),
            Direction::RIGHT => Some(direction.rotated_by(Rotor3::from_rotation_xz(ROTATION))),
            Direction::UP => Some(direction.rotated_by(Rotor3::from_angle_plane(
                ROTATION,
                Bivec3::from_normalized_axis(camera.right),
            ))),
            Direction::DOWN => Some(direction.rotated_by(Rotor3::from_angle_plane(
                -ROTATION,
                Bivec3::from_normalized_axis(camera.right),
            ))),
        };

        if let Some(new_direction) = new_direction {
            camera.position = camera.center + new_direction;

            // important
            camera.reset();
            self.renderer.set_camera(camera);
            self.should_reset_image = true;
        }
    }
}
