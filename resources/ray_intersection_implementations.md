# Ray Intersection Implementations
## Sphere
This is the original implementation:

```rust
    fn intersects(&self, ray: &Ray) -> bool {
        let oc = ray.origin - self.center;
    
        let a = ray.direction.mag_sq();
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.mag_sq() - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
    
        discriminant > 0.0
    }
```

Note how we can delegate the multiplication of `b` by `2` to the discrimant:

```rust
    fn intersects(&self, ray: &Ray) -> bool {
        let oc = ray.origin - self.center;
            
        let a = ray.direction.mag_sq();
        let b = oc.dot(ray.direction);
        let c = oc.mag_sq() - self.radius * self.radius;
        let discriminant = 4.0 * b * b - 4.0 * a * c;       // HERE
    
        discriminant > 0.0
    }
```

Further, we see that everything in the `discrimant` is multiplied by `4`, we can therefore strike it out and see:

```rust
    fn intersects(&self, ray: &Ray) -> bool {
        let oc = ray.origin - self.center;
            
        let a = ray.direction.mag_sq();
        let b = oc.dot(ray.direction);
        let c = oc.mag_sq() - self.radius * self.radius;
        let discriminant = b * b - a * c;                   // HERE
    
        discriminant > 0.0
    }
```

However, we basically compare `b * b` against `a * c`, so we can reduce to:

```rust
    fn intersects(&self, ray: &Ray) -> bool {
        let oc = ray.origin - self.center;
            
        let a = ray.direction.mag_sq();
        let b = oc.dot(ray.direction);
        let c = oc.mag_sq() - self.radius * self.radius;
       
        b * b > a * c                                       // HERE
    }
```
    
