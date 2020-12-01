use crate::configuration::Configuration;
use crate::render::renderer::Renderer;
use show_image::{WindowOptions, Window, make_window_full, KeyCode};
use std::time::Duration;

pub struct FastWindow {
    window: Window,
    config: Configuration,
    renderer: Renderer,
}

impl FastWindow {
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
        })
    }

    pub fn start_rendering(&mut self) {
        let wait_key = Duration::from_millis(100);
        let render_job = self.renderer.render_all();

        while let Ok(event) = self.window.wait_key(wait_key) {
            if let Some(event) = event {
                if event.key == KeyCode::Escape {
                    if self.config.verbose {
                        println!("Stopping render threads");
                    }
                    render_job.stop().expect("Could not stop render threads");
                    return;
                }
            }
            if self.renderer.is_done() {
                break;
            }

            let image = self.renderer.get_image_u8();
            if let Some(e) = self.window.set_image(image, "Rendering").err() {
                eprintln!("{}\nSkipping this image!", e);
            }
        }

        render_job.join().expect("Could not join render threads");

        let image = self.renderer.get_image_u8();
        self.window.set_image(image, "Rendering").expect("Could not set last image");

        // wait for user save or stop
        while let Ok(event) = self.window.wait_key(wait_key) {
            if let Some(event) = event {
                if event.key == KeyCode::Escape {
                    break;
                }
            }
        }
    }
}
