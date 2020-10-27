use crate::floats;

pub fn solve_quadratic(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    debug_assert!(!a.is_nan());
    debug_assert!(!b.is_nan());
    debug_assert!(!c.is_nan());

    if floats::lt_epsilon(a) {
        if floats::lt_epsilon(b) {
            return None;
        }

        let sol = -c / b;

        return Some((sol, sol));
    }

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }

    let a_x1 = -0.5 * (b + f32::copysign(discriminant.sqrt(), b));

    Some((a_x1 / a, c / a_x1))
}
