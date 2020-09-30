use ultraviolet::Vec3;

use crate::geometry::aabb::Aabb;
use crate::geometry::Boxable;

pub mod kd_tree;
pub mod uniform_spatial_partition;

fn global_bounding_box<T: Boxable>(objects: &Vec<T>) -> Option<Aabb> {
    if objects.is_empty() {
        return None;
    }

    let mut aabb = Aabb::default();
    let mut at_least_one_aabb = false;

    for o in objects {
        if let Some(boxed) = o.bounding_box() {
            if !at_least_one_aabb {
                at_least_one_aabb = true;
                aabb = boxed;
            } else {
                aabb = aabb.outer_join(&boxed);
            }
        }
    }

    if !at_least_one_aabb {
        return None;
    }

    Some(aabb)
}

fn average_cell_size<T: Boxable>(objects: &Vec<T>) -> Option<Vec3> {
    if objects.is_empty() {
        return None;
    }

    let mut cell_size = Vec3::zero();
    let mut at_least_one_cell = false;

    for o in objects {
        if let Some(aabb) = o.bounding_box() {
            at_least_one_cell = true;
            cell_size += aabb.size();
        }
    }

    if !at_least_one_cell {
        return None;
    }

    Some(cell_size / objects.len() as f32)
}
