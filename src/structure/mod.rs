pub mod kd_tree;
pub mod uniform_spatial_partition;

use crate::geometry::{Aabb, Boxable, Intersectable, Ray};
use ultraviolet::Vec3;

fn global_bounding_box<T: Boxable>(objects: &Vec<T>) -> Option<Aabb> {
    if objects.is_empty() {
        return None;
    }

    let mut aabb = Aabb::new(Vec3::one() * f32::INFINITY, Vec3::one() * f32::NEG_INFINITY);

    for o in objects {
        if let Some(boxed) = o.bounding_box() {
            aabb = aabb.outer_join(&boxed);
        }
    }

    if aabb.min.x == f32::INFINITY {
        return None;
    }

    Some(aabb)
}

fn bounding_box_and_cell_size<T: Boxable>(objects: &Vec<T>) -> (Option<Aabb>, Option<Vec3>) {
    if objects.is_empty() {
        return (None, None);
    }

    let mut bounding_box = Aabb::new(Vec3::one() * f32::INFINITY, Vec3::one() * f32::NEG_INFINITY);

    let mut cell_size = Vec3::one() * f32::INFINITY;

    for o in objects {
        if let Some(aabb) = o.bounding_box() {
            bounding_box = bounding_box.outer_join(&aabb);
            cell_size += aabb.size();
        }
    }

    if cell_size.x == f32::INFINITY {
        return (None, None);
    }

    cell_size /= objects.len() as f32;

    (Some(bounding_box), Some(cell_size))
}
