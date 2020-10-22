use ultraviolet::Vec3;

use crate::geometry::{Boxable, CeilFloorExt, Intersectable, Intersection};
use crate::geometry::aabb::Aabb;
use crate::geometry::ray::Ray;
use crate::render::objects::SceneObject;
use crate::render::Scene;
use crate::acceleration_structure::{AccelerationStructure, average_cell_size, global_bounding_box, check_intersection};

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

// TODO: This is a naive implementation: Make more performant
impl<'obj> AccelerationStructure for SpatialPartition<'obj> {
    fn accelerate(&self, ray: &Ray, scene: &Scene) -> Option<Intersection> {
        let mut intersections = Vec::new();

        for x in &self.grid {
            for y in x {
                for spatial_cell in y {
                    // hits spatial cell?
                    if spatial_cell.bounding_box.intersects(ray).is_some() {
                        for o in &spatial_cell.objects {
                            // hits object aabb?
                            if let Some(i) = check_intersection(ray, o) {
                                intersections.push(i);
                            }
                        }
                    }
                }
            }
        }

        if intersections.is_empty() {
            return None;
        }


        let i = intersections
            .iter()
            .min_by(|i0, i1| i0.t.unwrap().partial_cmp(&i1.t.unwrap()).unwrap())
            .unwrap();
        let clone = Intersection::new(
            i.position.unwrap(),
            i.normal.unwrap(),
            i.t.unwrap(),
        );
        Some(clone)
    }
}
