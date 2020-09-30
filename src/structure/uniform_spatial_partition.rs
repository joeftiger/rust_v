use ultraviolet::Vec3;

use crate::geometry::{Boxable, CeilFloorExt, Intersectable};
use crate::geometry::aabb::Aabb;
use crate::structure::{average_cell_size, global_bounding_box};

#[derive(Default)]
struct SpatialCell<'a, T: Boxable> {
    bounding_box: Aabb,
    objects: Vec<&'a T>,
}

struct SpatialPartition<'a, T: Boxable> {
    grid: Vec<Vec<Vec<SpatialCell<'a, T>>>>,
    cell_size: Vec3,
}

impl<'a, T: Boxable> SpatialPartition<'a, T> {
    pub fn new_averaged(boxables: &'a Vec<T>) -> SpatialPartition<'a, T> {
        let global_box = global_bounding_box(boxables).unwrap();
        let cell_size = average_cell_size(boxables).unwrap();

        // fit cells
        let fitting_cells = global_box.size() / cell_size;
        let cell_size = cell_size * fitting_cells.ceil() / fitting_cells;
        let fitting_cells = fitting_cells.ceil();

        let unit = Vec3::unit_x() * cell_size.x
            + Vec3::unit_y() * cell_size.y
            + Vec3::unit_z() * cell_size.z;

        // create grid
        let mut grid: Vec<Vec<Vec<SpatialCell<T>>>> = Vec::with_capacity(cell_size.x as usize);

        for x in 0..fitting_cells.x as i32 {
            let mut grid_y = Vec::with_capacity(cell_size.y as usize);

            for y in 0..fitting_cells.y as i32 {
                let mut grid_z = Vec::with_capacity(cell_size.z as usize);

                for z in 0..fitting_cells.z as i32 {
                    let index = Vec3::new(x as f32, y as f32, z as f32);

                    let min = global_box.min + unit * index;
                    let max = min + cell_size;

                    let bounding_box = Aabb::new(min, max);

                    // add objects inside the cell boundinx box
                    let mut objects = vec![];
                    for o in boxables {
                        if o.bounding_box()
                            .unwrap()
                            .intersects(&bounding_box)
                            .is_some()
                        {
                            objects.push(o);
                        }
                    }

                    let cell = SpatialCell {
                        bounding_box,
                        objects,
                    };
                    grid_z.push(cell);
                }

                grid_y.push(grid_z);
            }

            grid.push(grid_y);
        }

        Self { grid, cell_size }
    }
}
