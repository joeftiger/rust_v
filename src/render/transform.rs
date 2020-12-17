use ultraviolet::{Mat4, Vec3};
use util::floats;
use std::ops::Mul;

pub struct Transform {
    m: Mat4,
    m_inv: Mat4,
}

impl Transform {
    pub fn new(m: Mat4) -> Self {
        let m_inv = m.inversed();
        Self { m, m_inv }
    }

    pub fn inversed(&self) -> Self {
        Self {
            m: self.m_inv,
            m_inv: self.m,
        }
    }

    pub fn transposed(&self) -> Self {
        Self {
            m: self.m.transposed(),
            m_inv: self.m_inv.transposed(),
        }
    }

    pub fn is_identity(&self) -> bool {
        self.m == Mat4::identity()
    }

    pub fn get_matrix(&self) -> &Mat4 {
        &self.m
    }

    pub fn get_inverse_matrix(&self) -> &Mat4 {
        &self.m_inv
    }

    pub fn has_scale(&self) -> bool {
        !floats::approx_equal(self.m[0][0], 1.0) || !floats::approx_equal(self.m[1][1], 1.0) || !floats::approx_equal(self.m[2][2], 1.0)
    }

    pub fn has_translation(&self) -> bool {
        !floats::approx_zero(self.m[0][3])
            || !floats::approx_zero(self.m[1][3])
            || !floats::approx_zero(self.m[2][3])
    }

    pub fn has_rotation(&self) -> bool {
        !floats::approx_zero(self.m[0][1]) || !floats::approx_zero(self.m[0][2])
            || !floats::approx_zero(self.m[1][0]) || !floats::approx_zero(self.m[1][2])
            || !floats::approx_zero(self.m[2][0]) || !floats::approx_zero(self.m[2][1])
    }

    #[rustfmt::skip]
    pub fn translate(delta: &Vec3) -> Self {
        let m = Mat4::from([
            1.0, 0.0, 0.0, delta.x,
            0.0, 1.0, 0.0, delta.y,
            0.0, 0.0, 1.0, delta.z,
            0.0, 0.0, 0.0, 1.0]);
        let m_inv = Mat4::from([
            1.0, 0.0, 0.0, -delta.x,
            0.0, 1.0, 0.0, -delta.y,
            0.0, 0.0, 1.0, -delta.z,
            0.0, 0.0, 0.0, 1.0
        ]);

        Self { m, m_inv }
    }

    #[rustfmt::skip]
    pub fn scale(v: &Vec3) -> Self {
        let m = Mat4::from([
            v.x, 0.0, 0.0, 0.0,
            0.0, v.y, 0.0, 0.0,
            0.0, 0.0, v.z, 0.0,
            0.0, 0.0, 0.0, 1.0
        ]);
        let m_inv = Mat4::from([
            1.0 / v.x, 0.0,       0.0,       0.0,
            0.0,       1.0 / v.y, 0.0,       0.0,
            0.0,       0.0,       1.0 / v.z, 0.0,
            0.0,       0.0,       0.0,       1.0
        ]);

        Self { m, m_inv }
    }

    #[rustfmt::skip]
    pub fn rotate_x(theta: f32) -> Self {
        let sin = theta.sin();
        let cos = theta.cos();

        let m = Mat4::from([
            1.0, 0.0,  0.0, 0.0,
            0.0, cos, -sin, 0.0,
            0.0, sin,  cos, 0.0,
            0.0, 0.0,  0.0, 1.0
        ]);

        Self { m, m_inv: m.transposed() }
    }

    #[rustfmt::skip]
    pub fn rotate_y(theta: f32) -> Self {
        let sin = theta.sin();
        let cos = theta.cos();

        let m = Mat4::from([
             cos, 0.0, sin, 0.0,
             0.0, 1.0, 0.0, 0.0,
            -sin, 0.0, cos, 0.0,
             0.0, 0.0, 0.0, 1.0
        ]);

        Self { m, m_inv: m.transposed() }
    }

    #[rustfmt::skip]
    pub fn rotate_z(theta: f32) -> Self {
        let sin = theta.sin();
        let cos = theta.cos();

        let m = Mat4::from([
            cos, -sin, 0.0, 0.0,
            sin,  cos, 0.0, 0.0,
            0.0,  0.0, 1.0, 0.0,
            0.0,  0.0, 0.0, 1.0
        ]);

        Self { m, m_inv: m.transposed() }
    }

    pub fn rotate(theta: f32, axis: &Vec3) -> Self {
        let a = axis.normalized();
        let sin = theta.sin();
        let cos = theta.cos();

        let xx = a.x * a.x;
        let xy = a.x * a.y;
        let xz = a.x * a.z;
        let yy = a.y * a.y;
        let yz = a.y * a.z;
        let zz = a.z * a.z;

        let m = Mat4::from([
            xx + (1.0 - xx) * cos,
            xy * (1.0 - cos) - a.z * sin,
            xz * (1.0 - cos) + a.y * sin,
            0.0,
            xy * (1.0 - cos) + a.z * sin,
            yy + (1.0 - yy) * cos,
            yz * (1.0 - cos) - a.x * sin,
            0.0,
            xz * (1.0 - cos) - a.y * sin,
            yz * (1.0 - cos) + a.x * sin,
            zz + (1.0 - zz) * cos,
            0.0,
            0.0, 0.0, 0.0, 1.0
        ]);

        Self { m, m_inv: m.transposed() }
    }

    pub fn look_at(pos: &Vec3, look: &Vec3, up: &Vec3) -> Self {
        let dir = (*look - *pos).normalized();
        let right =  up.normalized().cross(dir);

        if right.mag() == 0.0 {
            panic!("'up' vector {:?} and viewing direction {:?} passed to look_at() are pointing in the same direction.", up, dir);
        }

        let new_up = dir.cross(right);

        let m_inv = Mat4::new(
            right.into_homogeneous_vector(),
            new_up.into_homogeneous_vector(),
            dir.into_homogeneous_vector(),
            pos.into_homogeneous_point()
        );

        Self { m: m_inv.inversed(), m_inv }
    }

    pub fn orthographic(z_near: f32, z_far: f32) -> Self {
        let scale = Self::scale(&Vec3::new(1.0, 1.0, 1.0 / (z_far - z_near)));
        let translation = Self::translate(&Vec3::new(0.0, 0.0, -z_near));

        scale * translation
    }

    pub fn perspective(fov: f32, n: f32, f: f32) -> Self {
        // Perform projective divide for perspective projection
        let m22 = f / (f - n);
        let m23 = -m22 * n;
        let perspective = Mat4::from([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, m22, m23,
            0.0, 0.0, 1.0, 0.0
        ]);

        // Scale canonical perspective view to specified field of view
        let inv_tan = 1.0 / (fov / 2.0).tan();

        let scale = Self::scale(&Vec3::new(inv_tan, inv_tan, 1.0));

        scale * Self::new(perspective)
    }
}

impl PartialEq for Transform {
    fn eq(&self, other: &Self) -> bool {
        self.m == other.m && self.m_inv == other.m_inv
    }
}

impl Mul for Transform {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            m: self.m * rhs.m,
            m_inv: rhs.m * self.m
        }
    }
}