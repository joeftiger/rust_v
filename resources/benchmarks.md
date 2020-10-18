# Benchmarks
The following benchmarks were made on an AMD Ryzen 5 1500X.

## Aabb - Ray Intersections (commit 15cc7b600873e5dbd2541d8242a63a7ad4ff1830)
**Stress tested random aabbs/rays for 300s** \
commit 15cc7b600873e5dbd2541d8242a63a7ad4ff1830:

| **Compilation** | *raycasts / s* |
| native          |     13'590'051 |
| native + pgo    |     14'200'338 |


## Aabb - Ray Intersections (OLD)
The different implementations operated on a `Vec3` struct or directly on `f32` values.

**Stress tested for 300s:**

| **Implementation** | *raycasts / s* |
|--------------------|----------------|
| direct `Vec3`      |     26'838'268 |
| written-out `f32`  |     26'922'912 |

## Sphere - Ray Intersections (OLD)
**Stress tested for 300s:**

| **Implementation** | *raycasts / s* |
|--------------------|----------------|
| direct `Vec3`      |     25'339'074 |
