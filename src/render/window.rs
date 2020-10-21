use std::time::{Duration, Instant};

use show_image::KeyCode::*;
use show_image::{make_window, make_window_full, KeyCode, Window, WindowOptions};
use ultraviolet::Rotor3;

use crate::render::Renderer;

const ROTATING_ANGLE_DEG: f32 = 22.5;
const ROTATING_ANGLE_RAD: f32 = (ROTATING_ANGLE_DEG as f64 * 0.017_453_292_519_943_295) as f32; // latter is PI / 180

const RENDER_TIME_MS: u64 = 500;

pub struct CustomWindow {
    window: Window,
    renderer: Box<dyn Renderer>,
}

impl CustomWindow {
    pub fn new(name: impl Into<String>, mut renderer: Box<dyn Renderer>) -> Result<Self, String> {
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

    pub fn take_control(&mut self) {
        self.take_control_dt(Duration::from_millis(RENDER_TIME_MS))
    }

    pub fn take_control_dt(&mut self, render_time: Duration) {
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

    fn render_loop_or_exit(&mut self, wait_key: Duration, render_time: Duration) -> bool {
        while let Ok(event) = self.window.wait_key(wait_key) {
            // handle input
            if let Some(event) = event {
                if event.key == Escape {
                    println!("Exiting window");
                    return true;
                }

                if let Some(rotation) = CustomWindow::create_rotation(event.key) {
                    println!("Updating camera");
                    self.update_camera_rotation(rotation);
                }
            }

            // handle rendering
            println!("Rendering");
            let start = Instant::now();
            let mut done = false;
            while start.elapsed() < render_time {
                if self.renderer.render_pass() {
                    println!("Rendering complete");
                    done = true;
                    break;
                }
            }

            // update rendering
            let image = self.renderer.get_image();
            self.window.set_image(image, "Rendering").unwrap();

            if done {
                return false;
            }
        }

        true
    }

    fn input_loop_or_exit(&mut self, wait_key: Duration) -> bool {
        while let Ok(event) = self.window.wait_key(wait_key) {
            // handle input
            if let Some(event) = event {
                if event.key == Escape {
                    println!("Exiting window");
                    return true;
                }

                if let Some(rotation) = CustomWindow::create_rotation(event.key) {
                    println!("Updating camera");
                    self.update_camera_rotation(rotation);
                    return false;
                }
            }
        }

        true
    }

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
