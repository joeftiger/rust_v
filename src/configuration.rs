use crate::cornell_box;
use crate::render::integrator::debug_normals::DebugNormals;
use crate::render::integrator::path::Path;
use crate::render::integrator::whitted::Whitted;
use crate::render::integrator::Integrator;
use crate::render::renderer::Renderer;
use crate::render::sampler::{NoopSampler, RandomSampler, Sampler};
use std::convert::TryInto;
use std::sync::Arc;
use std::time::Instant;
#[cfg(feature = "live-window")]
use crate::render::fast_window::FastWindow;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub verbose: bool,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub passes: u32,
    pub block_size: u32,
    pub live: bool,
    pub threaded: bool,
    pub output: Option<String>,
    pub pixel_format: PixelFormat,
    pub integrator_backend: IntegratorBackend,
}

impl Configuration {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        verbose: bool,
        width: u32,
        height: u32,
        depth: u32,
        passes: u32,
        block_size: u32,
        live: bool,
        threaded: bool,
        output: Option<String>,
        pixel_format: PixelFormat,
        integrator_backend: IntegratorBackend,
    ) -> Self {
        Self {
            verbose,
            width,
            height,
            depth,
            passes,
            block_size,
            live,
            threaded,
            output,
            pixel_format,
            integrator_backend,
        }
    }

    /// Creates a renderer instance from this configuration file.
    pub fn create_renderer(&self) -> Renderer {
        let (scene, camera) = cornell_box::create(self.width, self.height);
        // let (scene, camera) = plain_scene::create(self.width, self.height);

        let integrator: Arc<dyn Integrator> = match self.integrator_backend {
            IntegratorBackend::Debug => Arc::new(DebugNormals),
            IntegratorBackend::Whitted => Arc::new(Whitted::new(self.depth)),
            IntegratorBackend::Path => Arc::new(Path::new(3, self.depth)),
        };

        let sampler: Arc<dyn Sampler> = match self.integrator_backend {
            IntegratorBackend::Debug => Arc::new(NoopSampler),
            _ => Arc::new(RandomSampler::default()),
        };

        Renderer::new(
            Arc::new(scene),
            Arc::new(camera),
            sampler,
            integrator,
            self.block_size,
            self.passes
        )
    }

    pub fn start_rendering(&'static self) -> Result<(), String> {
        if self.verbose {
            println!("{:#?}", self);
        }

        let mut renderer = self.create_renderer();

        #[cfg(feature = "live-window")]
        {
            if self.live {
                FastWindow::new("Rust-V".to_string(), self.clone(), renderer)
                    .map_err(|e| format!("Unable to create window: {}", e))?
                    .start_rendering();
                if self.verbose {
                    println!("Closed window");
                }
                return Ok(());
            }
        }

        if cfg!(not(feature = "live-window")) || !self.live {
            let start = Instant::now();

            let job = renderer.render_all();
            job.join().expect("Could not join render threads");

            if self.verbose {
                println!("Took {} seconds", start.elapsed().as_secs());
            }
        }

        if let Some(output) = &self.output {
            if self.verbose {
                println!("Saving image");
            }
            match self.pixel_format {
                PixelFormat::U8 => renderer
                    .get_image_u8()
                    .save(output)
                    .map_err(|e| format!("Unable to save image: {}", e))?,
                PixelFormat::U16 => renderer
                    .get_image_u16()
                    .save(output)
                    .map_err(|e| format!("Unable to save image: {}", e))?,
            };
            println!("Successfully saved image");
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum PixelFormat {
    U8,
    U16,
}

impl TryInto<PixelFormat> for &str {
    type Error = String;

    fn try_into(self) -> Result<PixelFormat, Self::Error> {
        match self {
            "u8" | "U8" => Ok(PixelFormat::U8),
            "u16" | "U16" => Ok(PixelFormat::U16),
            _ => Err(self.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum IntegratorBackend {
    Debug,
    Whitted,
    Path,
}

impl TryInto<IntegratorBackend> for &str {
    type Error = String;

    fn try_into(self) -> Result<IntegratorBackend, Self::Error> {
        match self {
            "debug" | "Debug" | "DEBUG" => Ok(IntegratorBackend::Debug),
            "whitted" | "Whitted" | "WHITTED" => Ok(IntegratorBackend::Whitted),
            "path" | "Path" | "PATH" => Ok(IntegratorBackend::Path),
            _ => Err(self.to_string()),
        }
    }
}
