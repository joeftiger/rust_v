use std::ops::{Range, Index};
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Block {
    x: Range<i64>,
    y: Range<i64>,
}

impl Block {
    pub fn new(x: Range<i64>, y: Range<i64>) -> Self {
        Self { x, y }
    }

    pub fn prod(&self) -> Vec<(i64, i64)> {
        self.x.clone().flat_map(|x| self.y.clone().map(move |y| (x, y))).collect()
    }
}

pub struct RangeBlock {
    pub width: u32,
    pub height: u32,
    pub block_border_size: u32,
    pub block_width: u32,
    pub block_height: u32,
    blocks: Vec<Block>,
    index: AtomicUsize,
}

impl RangeBlock {
    pub fn new(width: u32, height: u32, block_border_size: u32) -> Self {
        let block_width = (width as f32 / block_border_size as f32).ceil() as u32;
        let block_height = (height as f32 / block_border_size as f32).ceil() as u32;
        let mut blocks = Vec::with_capacity((block_width * block_height) as usize);

        let x_max = width as i64;
        let y_max = height as i64;

        for y in 0..block_height {
            for x in 0..block_width {
                let x_start = (x * block_border_size) as i64;
                let y_start = (y * block_border_size) as i64;

                let x_end = x_start + block_border_size as i64;
                let y_end = y_start + block_border_size as i64;

                let x_range = x_start..(x_max.min(x_end));
                let y_range = y_start..(y_max.min(y_end));

                blocks.push(Block::new(x_range, y_range));
            }
        }

        let index = AtomicUsize::new(0);

        Self { width, height, block_border_size, block_width, block_height, blocks, index }
    }

    pub fn next_block(&mut self) -> &Block {
        let index = self.index.fetch_add(1, Ordering::SeqCst);
        &self.blocks[index]
    }

    pub fn num_blocks(&self) -> usize {
        self.blocks.len()
    }
}

impl Index<usize> for RangeBlock {
    type Output = Block;

    fn index(&self, index: usize) -> &Self::Output {
        &self.blocks[index]
    }
}
