use ultraviolet::{Rotor3, Vec3};

fn main() {
    rotor_bug();
}

fn rotor_bug() {
    let from = Vec3::unit_x();
    let to = -from;

    let rotor = Rotor3::from_rotation_between(from, to);
    println!("{:?}", rotor);
}
