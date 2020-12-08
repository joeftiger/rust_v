use crate::demos::*;
#[cfg(feature = "live-window")]
use crate::render::fast_window::FastWindow;
use crate::render::integrator::debug_normals::DebugNormals;
use crate::render::integrator::path::Path;
use crate::render::integrator::whitted::Whitted;
use crate::render::integrator::Integrator;
use crate::render::renderer::Renderer;
use crate::render::sampler::{NoopSampler, RandomSampler, Sampler};
use std::convert::TryInto;
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub verbose: bool,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub passes: u32,
    pub block_size: u32,
    pub live: bool,
    pub threads: u32,
    pub output: Option<String>,
    pub pixel_type: PixelType,
    pub integrator_type: IntegratorType,
    pub demo_type: DemoType,
}

impl Configuration {
    /// Creates a renderer instance from this configuration file.
    pub fn create_renderer(&self) -> Renderer {
        let (scene, camera) = match self.demo_type {
            DemoType::Spheres => Spheres::create(self.width, self.height),
            DemoType::Cornell => CornellBox::create(self.width, self.height),
        };
        let scene = Arc::new(scene);
        let camera = Arc::new(camera);

        let integrator: Arc<dyn Integrator> = match self.integrator_type {
            IntegratorType::Debug => Arc::new(DebugNormals),
            IntegratorType::Whitted => Arc::new(Whitted::new(self.depth)),
            IntegratorType::Path => Arc::new(Path::new(3, self.depth)),
        };

        let sampler: Arc<dyn Sampler> = match self.integrator_type {
            IntegratorType::Debug => Arc::new(NoopSampler),
            _ => Arc::new(RandomSampler::default()),
        };

        let config = Arc::new(self.clone());

        Renderer::new(scene, camera, sampler, integrator, config)
    }

    #[cfg(feature = "hpc-signals")]
    fn signal_watcher(&'static self, renderer: Renderer) {
        unsafe {
            signal_hook::register(signal_hook::SIGTERM, move || {
                println!("Received SIGTERM. Saving current image...");

                match self.save_image(&renderer) {
                    Ok(()) => std::process::exit(0),
                    Err(err) => {
                        println!("{}", err);
                        std::process::exit(-1);
                    }
                }
            })
            .unwrap();
        }
    }

    fn save_image(&self, renderer: &Renderer) -> Result<(), String> {
        if let Some(output) = &self.output {
            if self.verbose {
                println!("Saving image");
            }
            match self.pixel_type {
                PixelType::U8 => renderer
                    .get_image_u8()
                    .save(output)
                    .map_err(|e| format!("Unable to save image: {}", e))?,
                PixelType::U16 => renderer
                    .get_image_u16()
                    .save(output)
                    .map_err(|e| format!("Unable to save image: {}", e))?,
            };
            println!("Successfully saved image");
        }

        Ok(())
    }

    pub fn start_rendering(&'static self) -> Result<(), String> {
        if self.verbose {
            println!("{:#?}", self);
        }

        let mut renderer = self.create_renderer();
        #[cfg(feature = "hpc-signals")]
        {
            self.signal_watcher(renderer.clone());
        }

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

        self.save_image(&renderer)
    }
}

#[derive(Debug, Clone)]
pub enum PixelType {
    U8,
    U16,
}

impl TryInto<PixelType> for &str {
    type Error = String;

    fn try_into(self) -> Result<PixelType, Self::Error> {
        match self {
            "u8" | "U8" => Ok(PixelType::U8),
            "u16" | "U16" => Ok(PixelType::U16),
            _ => Err(self.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum IntegratorType {
    Debug,
    Whitted,
    Path,
}

impl TryInto<IntegratorType> for &str {
    type Error = String;

    fn try_into(self) -> Result<IntegratorType, Self::Error> {
        match self {
            "debug" | "Debug" | "DEBUG" => Ok(IntegratorType::Debug),
            "whitted" | "Whitted" | "WHITTED" => Ok(IntegratorType::Whitted),
            "path" | "Path" | "PATH" => Ok(IntegratorType::Path),
            _ => Err(self.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DemoType {
    Spheres,
    Cornell,
}

impl TryInto<DemoType> for &str {
    type Error = String;

    fn try_into(self) -> Result<DemoType, Self::Error> {
        match self {
            "spheres" | "Spheres" | "SPHERES" => Ok(DemoType::Spheres),
            "cornell" | "Cornell" | "CORNELL" => Ok(DemoType::Cornell),
            _ => Err(self.to_string()),
        }
    }
}
