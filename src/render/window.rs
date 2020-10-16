use std::time::{Duration, Instant};

use show_image::{make_window, Window};
use show_image::KeyCode::*;
use ultraviolet::{Rotor3, Vec3};

use crate::render::Renderer;

/// The rotating angle speed in radians
const ROTATING_ANGLE_DEG: f32 = 22.5;
const ROTATING_ANGLE: f32 = ROTATING_ANGLE_DEG * 0.017_453_292; // latter is PI / 180

const RENDER_TIME_MS: u64 = 250;

pub struct CustomWindow {
    window: Window,
    renderer: Box<dyn Renderer>,
}

impl CustomWindow {
    pub fn new(name: impl Into<String>, renderer: Box<dyn Renderer>) -> Result<Box<Self>, String> {
        let cw = Self {
            window: make_window(name)?,
            renderer,
        };

        Result::Ok(Box::new(cw))
    }

    pub fn take_control(&mut self, dt: Duration) {
        let wait_key = Duration::from_millis(1);

        while let Ok(event) = self.window.wait_key(wait_key) {
            if let Some(event) = event {
                let key = event.key;
                if key == Escape {
                    return;
                }

                let mut rotation: Option<Rotor3> = None;

                // rotate YAW
                if key == ArrowLeft {
                    rotation = Some(Rotor3::from_rotation_xz(-ROTATING_ANGLE));
                } else if key == ArrowRight {
                    rotation = Some(Rotor3::from_rotation_xz(ROTATING_ANGLE));
                }

                // rotate PITCH
                if key == ArrowUp {
                    let r = Rotor3::from_euler_angles(0.0, ROTATING_ANGLE, 0.0);
                    if let Some(yaw) = rotation {
                        rotation = Some(yaw * r);   // note the order of multiplication
                    } else {
                        rotation = Some(r);
                    }
                } else if key == ArrowDown {
                    let r = Rotor3::from_euler_angles(0.0, -ROTATING_ANGLE, 0.0);
                    if let Some(yaw) = rotation {
                        rotation = Some(yaw * r);
                    } else {
                        rotation = Some(r);
                    }
                }

                // apply rotation
                let camera = &mut self.renderer.get_scene().camera;
                if let Some(rotation) = rotation {
                    let mut distance = camera.camera_info.position - camera.camera_info.center;
                    distance.rotate_by(rotation);

                    camera.camera_info.position = camera.camera_info.center + distance;

                    // IMPORTANT
                    camera.update();
                    self.renderer.reset();
                }
            }

            let start = Instant::now();
            while (Instant::now() - start) < Duration::from_millis(RENDER_TIME_MS) {
                self.renderer.render_pass();
            }

            let image = self.renderer.get_image();
            self.window.set_image(image, "Rendering").unwrap();
        }
    }
}
