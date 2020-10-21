use ultraviolet::Vec3;

use crate::geometry::{Boxable, CeilFloorExt, Intersectable, Intersection};
use crate::geometry::aabb::Aabb;
use crate::structure::{average_cell_size, global_bounding_box, AccelerationStructure};
use crate::geometry::ray::Ray;
use crate::render::Scene;
use crate::render::objects::SceneObject;

#[derive(Default)]
struct SpatialCell<T> {
    bounding_box: Aabb,
    objects: Vec<T>,
}

struct SpatialPartition<'obj> {
    grid: Vec<Vec<Vec<SpatialCell<&'obj Box<dyn SceneObject>>>>>,
    cell_size: Vec3,
}

impl<'obj> SpatialPartition<'obj> {
    pub fn new_averaged(scene: &'obj Scene) -> Self {
        let boxables = &scene.objects;
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
        let mut grid: Vec<Vec<Vec<SpatialCell<&'obj Box<dyn SceneObject>>>>> = Vec::with_capacity(cell_size.x as usize);

        for x in 0..fitting_cells.x as i32 {
            let mut grid_y = Vec::with_capacity(cell_size.y as usize);

            for y in 0..fitting_cells.y as i32 {
                let mut grid_z = Vec::with_capacity(cell_size.z as usize);

                for z in 0..fitting_cells.z as i32 {
                    let index = Vec3::new(x as f32, y as f32, z as f32);

                    let min = global_box.min + unit * index;
                    let max = min + cell_size;

                    let bounding_box = Aabb::new(min, max);

                    // add objects inside the cell bounding box
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

impl<'obj> AccelerationStructure<'obj> for SpatialPartition<'obj> {
    fn accelerate(&self, ray: &Ray, scene: &'obj Scene) -> Option<Intersection> {
        unimplemented!()
    }
}
