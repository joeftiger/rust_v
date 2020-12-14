use std::sync::{Arc, Mutex, RwLock};

use image::{ImageBuffer, Rgb};
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use lazy_static::lazy_static;

use color::Color;
use util::range_block::{Block, RangeBlock};

use crate::configuration::Configuration;
use crate::render::camera::Camera;
use crate::render::integrator::Integrator;
use crate::render::sampler::Sampler;
use crate::render::scene::Scene;
use crate::Spectrum;
use bitflags::_core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::thread::JoinHandle;

lazy_static! {
    pub static ref PROGRESS_BAR: Mutex<ProgressBar> =
        {
            let bar = ProgressBar::new(0);
            bar.set_style(ProgressStyle::default_bar().template(
                "[{elapsed} elapsed] {wide_bar:.cyan/white} {percent}% [{eta} remaining]",
            ));
            Mutex::new(bar)
        };
}

pub struct RenderJob<T> {
    should_stop: Arc<AtomicBool>,
    handles: Vec<JoinHandle<T>>,
}

impl<T: Default> RenderJob<T> {
    pub fn new(should_stop: Arc<AtomicBool>, handles: Vec<JoinHandle<T>>) -> Self {
        Self {
            should_stop,
            handles,
        }
    }

    pub fn stop(self) -> thread::Result<T> {
        self.should_stop.store(true, Ordering::SeqCst);
        for handle in self.handles {
            handle.join()?;
        }

        Ok(T::default())
    }

    pub fn join(self) -> thread::Result<T> {
        for job in self.handles {
            job.join()?;
        }

        Ok(T::default())
    }
}

#[derive(Default)]
struct SpectrumStatistic {
    x: u32,
    y: u32,
    spectrum: Spectrum,
    samples: usize,
}

impl SpectrumStatistic {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
            spectrum: Spectrum::black(),
            samples: 0,
        }
    }
    pub fn average(&self) -> Spectrum {
        if self.samples == 0 {
            self.spectrum
        } else {
            self.spectrum / self.samples as f32
        }
    }

    pub fn reset(&mut self) {
        self.samples = 0;
        self.spectrum = Spectrum::black();
    }
}

struct RenderBlock {
    stats: Vec<SpectrumStatistic>,
}

impl RenderBlock {
    pub fn reset(&mut self) {
        self.stats.iter_mut().for_each(|s| s.reset());
    }
}

impl From<&Block> for RenderBlock {
    fn from(block: &Block) -> Self {
        let stats = block
            .prod()
            .iter()
            .map(|(x, y)| SpectrumStatistic::new(*x as u32, *y as u32))
            .collect();
        Self { stats }
    }
}

#[derive(Clone)]
#[allow(clippy::rc_buffer)]
pub struct Renderer {
    scene: Arc<Scene>,
    camera: Arc<Camera>,
    sampler: Arc<dyn Sampler>,
    integrator: Arc<dyn Integrator>,
    render_blocks: Arc<Vec<Mutex<RenderBlock>>>,
    rendering: Arc<RwLock<ImageBuffer<Rgb<u8>, Vec<u8>>>>,
    progress: Arc<AtomicUsize>,
    config: Arc<Configuration>,
}

impl Renderer {
    pub fn new(
        scene: Arc<Scene>,
        camera: Arc<Camera>,
        sampler: Arc<dyn Sampler>,
        integrator: Arc<dyn Integrator>,
        config: Arc<Configuration>,
    ) -> Self {
        let (img_width, img_height) = (camera.width, camera.height);

        let range_block = RangeBlock::new(img_width, img_height, config.block_size);
        let render_blocks = range_block
            .blocks
            .iter()
            .map(|block| Mutex::new(RenderBlock::from(block)))
            .collect();

        Self {
            scene,
            camera,
            sampler,
            integrator,
            progress: Arc::new(AtomicUsize::new(0)),
            render_blocks: Arc::new(render_blocks),
            rendering: Arc::new(RwLock::new(ImageBuffer::new(img_width, img_height))),
            config,
        }
    }

    pub fn num_blocks(&self) -> usize {
        self.render_blocks.len()
    }

    pub fn num_pixels(&self) -> u32 {
        self.config.width * self.config.height
    }

    pub fn get_progress(&self) -> usize {
        self.progress.load(Ordering::Relaxed)
    }

    pub fn is_done(&self) -> bool {
        self.get_progress() >= self.num_blocks() * self.config.passes as usize
    }

    fn render(&self, x: u32, y: u32) -> Spectrum {
        let sample = self.sampler.get_2d();
        let ray = self.camera.primary_ray(x, y, &sample);

        self.integrator
            .integrate(&self.scene, &ray, self.sampler.clone())
    }

    pub fn reset_progress(&mut self) {
        self.progress.store(0, Ordering::Relaxed);
    }

    pub fn reset_image(&mut self) {
        self.render_blocks
            .iter()
            .for_each(|b| b.lock().expect("Block is poisoned").reset());
    }

    pub fn render_all(&mut self) -> RenderJob<()> {
        if self.is_done() {
            self.reset_progress()
        }

        // reset progress bar
        {
            let bar = PROGRESS_BAR.lock().expect("Progress bar poisoned");
            bar.set_length(self.num_blocks() as u64 * self.config.passes as u64);
            bar.reset();
        }

        let num_threads = self.config.threads;
        let mut handles = Vec::with_capacity(num_threads as usize);

        let should_stop = Arc::new(AtomicBool::new(false));
        for _ in 0..num_threads {
            let this = self.clone();
            let this_should_stop = should_stop.clone();

            let handle: JoinHandle<()> = thread::spawn(move || loop {
                if this_should_stop.load(Ordering::Relaxed) {
                    break;
                }

                let index = this.progress.fetch_add(1, Ordering::Relaxed);
                if index >= this.num_blocks() * this.config.passes as usize {
                    break;
                }
                let index = index % this.num_blocks();

                let mut lock = this.render_blocks[index].lock().expect("Block is poisoned");
                lock.stats.iter_mut().for_each(|stats| {
                    if stats.x == this.config.width / 2 && stats.y == this.config.height / 2 {
                        debug_assert_eq!(1, 1);
                    }

                    let pixel = this.render(stats.x, stats.y); //.clamp(0.0, 1.0); // FIXME
                    stats.spectrum += pixel;
                    stats.samples += 1;

                    let avg = stats.average().into();
                    this.rendering
                        .write()
                        .expect("Rendering poisoned")
                        .put_pixel(stats.x, stats.y, avg);
                });

                PROGRESS_BAR.lock().expect("Progress bar poisoned").inc(1);
            });

            handles.push(handle);
        }

        RenderJob::new(should_stop, handles)
    }

    pub fn get_image_u8(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        self.rendering
            .read()
            .expect("Rendering is poisoned")
            .clone()
    }

    // TODO: Possible to make more efficient?
    pub fn get_image_u16(&self) -> ImageBuffer<Rgb<u16>, Vec<u16>> {
        let mut buffer = ImageBuffer::new(self.config.width, self.config.height);
        self.render_blocks.iter().for_each(|block| {
            let lock = block.lock().expect("Block is poisoned");

            lock.stats
                .iter()
                .for_each(|stat| buffer.put_pixel(stat.x, stat.y, stat.average().into()));
        });

        buffer
    }
}
