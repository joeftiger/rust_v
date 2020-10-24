// use ultraviolet::Vec3;
//
// pub trait BSDF<TNormal, TIn, TOut> {
//     fn apply(
//         &self,
//         normal_in: TNormal,
//         normal_out: TNormal,
//         ray_in: TIn,
//         ray_out: TIn,
//     ) -> TOut;
// }
//
// pub struct Phong {
//     ambient: Vec3,
//     diffuse: Vec3,
//     specular: Vec3,
//     shininess: f32,
// }
//
// impl Phong {
//     pub fn new(ambient: Vec3, diffuse: Vec3, specular: Vec3, shininess: f32) -> Self {
//         Self {
//             ambient,
//             diffuse,
//             specular,
//             shininess,
//         }
//     }
// }
