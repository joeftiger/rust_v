use crate::floats;

// pub enum Solution {
//     Zero,
//     One(f32),
//     Two(f32, f32)
// }
//
//
//
// impl Iterator for Solution {
//     type Item = f32;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         match self {
//             Self::Zero => None,
//             Self::One(found) => {
//                 let found = *found;
//                 *self = Self::One(found);
//                 Some(found)
//             }
//             Self::Two(first, second) => {
//                 let first = *first;
//                 let second = *second;
//                 *self = Self::One(second);
//                 Some(first)
//             }
//         }
//     }
// }

pub fn solve_quadratic(a: f32, b: f32, c: f32) -> Vec<f32> {
    debug_assert!(!a.is_nan());
    debug_assert!(!b.is_nan());
    debug_assert!(!c.is_nan());

    if floats::lt_epsilon(a) {
        if floats::lt_epsilon(b) {
            return vec![];
            // return Solution::Zero;
        }

        return vec![-c / b];
        // return Solution::One(-c / b);
    }

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return vec![];
        // return Solution::Zero;
    }

    let a_x1 = -0.5 * (b + f32::copysign(discriminant.sqrt(), b));

    vec![a_x1 / a, c / a_x1]
    // Solution::Two(a_x1 / a, c / a_x1)
}
