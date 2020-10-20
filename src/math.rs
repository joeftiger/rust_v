use crate::util::floats;

pub fn solve_quadratic(a: f32, b: f32, c: f32) -> Vec<f32> {
    if floats::approx_zero(a) {
        if floats::approx_zero(b) {
            return vec![];
        }
        return vec![-c / b];
    }

    let discrimant = b * b - 4.0 * a * c;
    if discrimant < 0.0 {
        return vec![];
    }

    let a_x1 = -0.5 * (b * f32::copysign(discrimant.sqrt(), b));

    vec![a_x1 / a, c / a_x1]
}
