# Benchmarks
## Aabb - Ray Intersections
The different implementations operated on a `Vec3` struct or directly on `f32` values.

**Stress tested for 300s:**

| **Implementation** | *raycasts / s* |
|--------------------|----------------|
| direct `Vec3`      |     26'647'179 |
| written-out `f32`  |     26'922'912 |

## Sphere - Ray Intersections
**Stress tested for 300s:**

| **Implementation** | *raycasts / s* |
|--------------------|----------------|
| direct `Vec3`      |     25'339'074 |
