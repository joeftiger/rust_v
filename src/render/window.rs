use show_image::{make_window_full, KeyCode, Window, WindowOptions};

use crate::render::renderer::Renderer;
use std::time::{Duration, Instant};

const WAIT_KEY_MS: u64 = 1;
const RENDER_TIME_MS: u64 = 500;

pub struct RenderWindow<T> {
    window: Window,
    renderer: T,
    should_exit: bool,
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
                if event.key == KeyCode::Escape {
                    self.should_exit = true;
                    return;
                }
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
            if let Some(event) = event {
                if event.key == KeyCode::Escape {
                    self.should_exit = true;
                    return;
                }

                if event.key == KeyCode::Backspace {
                    self.renderer.reset();
                    return;
                }
            }
        }
    }
}
