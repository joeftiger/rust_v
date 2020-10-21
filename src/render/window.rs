use std::time::{Duration, Instant};

use show_image::KeyCode::*;
use show_image::{make_window, make_window_full, KeyCode, Window, WindowOptions, KeyboardEvent};
use ultraviolet::Rotor3;

use crate::render::Renderer;

const ROTATING_ANGLE_DEG: f32 = 22.5;
const ROTATING_ANGLE_RAD: f32 = (ROTATING_ANGLE_DEG as f64 * 0.017_453_292_519_943_295) as f32; // latter is PI / 180

const RENDER_TIME_MS: u64 = 500;

/// Creates a rotation from the key code according to ROTATING_ANGLE_RAD.
/// If the key code is not associated to arrow-[left, right, up, down], None will be returned.
fn create_rotation(key: KeyCode) -> Option<Rotor3> {
    let mut rotation: Option<Rotor3> = None;

    // rotate YAW
    if key == ArrowLeft {
        rotation = Some(Rotor3::from_rotation_xz(-ROTATING_ANGLE_RAD));
    } else if key == ArrowRight {
        rotation = Some(Rotor3::from_rotation_xz(ROTATING_ANGLE_RAD));
    }

    // rotate PITCH
    if key == ArrowUp {
        let r = Rotor3::from_euler_angles(0.0, ROTATING_ANGLE_RAD, 0.0);
        if let Some(yaw) = rotation {
            rotation = Some(yaw * r); // note the order of multiplication
        } else {
            rotation = Some(r);
        }
    } else if key == ArrowDown {
        let r = Rotor3::from_euler_angles(0.0, -ROTATING_ANGLE_RAD, 0.0);
        if let Some(yaw) = rotation {
            rotation = Some(yaw * r);
        } else {
            rotation = Some(r);
        }
    }

    rotation
}

pub struct CustomWindow<T: Renderer> {
    window: Window,
    renderer: T,
}

impl<T: Renderer> CustomWindow<T> {
    pub fn new(name: impl Into<String>, mut renderer: T) -> Result<Self, String> {
        let size = &renderer.get_camera().image_size;
        let options = WindowOptions::default()
            .set_name(name.into())
            .set_size([size.width, size.height]);

        let cw = Self {
            window: make_window_full(options)?,
            renderer,
        };

        Result::Ok(cw)
    }

    pub fn start_rendering(&mut self) {
        self.start_rendering_dt(Duration::from_millis(RENDER_TIME_MS))
    }

    pub fn start_rendering_dt(&mut self, render_time: Duration) {
        let wait_key = Duration::from_millis(1);

        loop {
            if self.render_loop_or_exit(wait_key, render_time) {
                break;
            }
            if self.input_loop_or_exit(wait_key) {
                break;
            }
        }
    }

    /// Enters the rendering loop or exits according to keyboard input.
    ///
    /// # Returns
    /// - bool: Exit and close the window
    fn render_loop_or_exit(&mut self, wait_key: Duration, render_time: Duration) -> bool {
        while let Ok(event) = self.window.wait_key(wait_key) {
            if self.handle_input_or_exit(event) {
                return true;
            }

            // handle rendering
            println!("Rendering for at most {} seconds...", render_time.as_secs_f32());
            let start = Instant::now();
            let mut done = false;
            while start.elapsed() < render_time {
                if self.renderer.render_pass() {
                    println!("Rendering complete");
                    done = true;
                    break;
                }
            }
            println!("Rendered for {} seconds", start.elapsed().as_secs_f32());

            // update rendering
            let image = self.renderer.get_image();
            self.window.set_image(image, "Rendering").unwrap();

            if done {
                return false;
            }
        }

        true
    }

    /// After the rendering is complete, enter the input loop or exit according to keyboard input.
    ///
    /// # Returns
    /// - bool: Exit and close the window
    fn input_loop_or_exit(&mut self, wait_key: Duration) -> bool {
        while let Ok(event) = self.window.wait_key(wait_key) {
            if self.handle_input_or_exit(event) {
                return true;
            }
        }

        true
    }

    /// Handles the keyboard event to exit or rotate the camera according to keyboard input.
    /// # Returns
    /// - bool: Exit and close the window
    fn handle_input_or_exit(&mut self, event: Option<KeyboardEvent>) -> bool {
        if let Some(event) = event {
            if event.key == Escape {
                println!("Exiting window");
                return true;
            }

            if let Some(rotation) = create_rotation(event.key) {
                println!("Updating camera");
                self.update_camera_rotation(rotation);
            }
        }

        false
    }

    /// Updates the camera rotation and resets the renderer
    fn update_camera_rotation(&mut self, rotation: Rotor3) {
        let camera = &mut self.renderer.get_scene().camera;
        let info = &mut camera.camera_info;

        let mut distance = info.position - info.center;
        distance.rotate_by(rotation);

        info.position = info.center + distance;

        // IMPORTANT
        camera.update();
        self.renderer.reset();
    }
}
