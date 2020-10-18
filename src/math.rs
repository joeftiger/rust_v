use crate::Float;

pub fn solve_quadratic(a: Float, b: Float, c: Float) -> Vec<Float> {
    if a.abs() <= Float::EPSILON {
        if b.abs() <= Float::EPSILON {
            return vec![];
        }
        return vec![-c / b];
    }

    let discrimant = b * b - 4.0 * a * c;
    if discrimant < 0.0 {
        return vec![];
    }

    let a_x1 = -0.5 * (b * Float::copysign(discrimant.sqrt(), b));

    vec![a_x1 / a, c / a_x1]
}
