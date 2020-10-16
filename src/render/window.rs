use std::time::Duration;

use show_image::{make_window, Window};
use show_image::KeyCode::*;

use crate::render::Renderer;
use ultraviolet::{Vec3, Rotor3};

/// The rotating angle speed in radians
const ROTATING_ANGLE_DEG: f32 = 22.5;
const ROTATING_ANGLE: f32 = ROTATING_ANGLE_DEG * 0.017_453_292; // latter is PI / 180

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
                let cam_info = &mut self.renderer.get_scene().camera.camera_info;
                if let Some(rotation) = rotation {
                    let mut distance = cam_info.position - cam_info.center;
                    distance.rotate_by(rotation);

                    cam_info.position = cam_info.center + distance;
                }
            }


        }
    }
}
