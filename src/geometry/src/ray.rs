use ultraviolet::Vec3;

#[derive(Clone, Copy, Debug, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub t_start: f32,
    pub t_end: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self::with(origin, direction, 0.0, f32::INFINITY)
    }

    pub fn with(origin:Vec3, direction: Vec3, t_start: f32, t_end: f32) -> Self {
        Self { origin, direction, t_start, t_end }
    }

    pub fn in_range(from: &Vec3, to: &Vec3) -> Self {
        let origin = *from;
        let mut direction = *to - *from;
        let t_start = 0.0;
        let t_end = direction.mag();
        direction.normalize();

        Self { origin, direction, t_start, t_end }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.direction.mul_add(Vec3::broadcast(t), self.origin)
    }

    #[inline(always)]
    pub fn is_in_range(&self, t: f32) -> bool {
        t >= self.t_start && t <= self.t_end
    }

    #[inline]
    pub fn is_in_range_op(&self, t: f32) -> Option<f32> {
        if self.is_in_range(t) {
            Some(t)
        } else {
            None
        }
    }
}

impl PartialEq for Ray {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin
            && self.direction == other.direction
            && self.t_start == other.t_start
            && self.t_end == other.t_end
    }
}