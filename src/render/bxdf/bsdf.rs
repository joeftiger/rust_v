use ultraviolet::{Vec2, Vec3};

use crate::render::bxdf::{same_hemisphere, world_to_bxdf, BxDF, BxDFSample, BxDFType};
use crate::Spectrum;
use std::ops::Deref;

pub struct BSDF {
    bxdfs: Vec<Box<dyn BxDF>>,
}

impl BSDF {
    pub fn new(bxdfs: Vec<Box<dyn BxDF>>) -> Self {
        Self { bxdfs }
    }

    pub fn num_bxdfs(&self) -> usize {
        self.bxdfs.len()
    }

    pub fn is_type(&self, t: BxDFType) -> bool {
        self.bxdfs.iter().any(|bxdf| bxdf.is_type(t))
    }

    pub fn num_types(&self, t: BxDFType) -> usize {
        self.bxdfs.iter().filter(|bxdf| bxdf.is_type(t)).count()
    }

    fn random_matching_bxdf(&self, t: BxDFType) -> &dyn BxDF {
        let count = self.num_types(t);
        let index = (fastrand::f32() * count as f32) as usize;

        println!("{:?}", t);

        let bxdf = self.bxdfs.iter().filter(|bxdf| bxdf.is_type(t)).nth(index);
        debug_assert!(bxdf.is_some());

        bxdf.unwrap().deref()
    }

    pub fn evaluate(
        &self,
        normal: &Vec3,
        incident_world: &Vec3,
        outgoing_world: &Vec3,
        mut types: BxDFType,
    ) -> Spectrum {
        let rotation = world_to_bxdf(normal);
        let incident = rotation * *incident_world;
        let outgoing = rotation * *outgoing_world;

        // transmission or reflection
        if same_hemisphere(&incident, &outgoing) {
            types = !types & BxDFType::TRANSMISSION;
        } else {
            types = !types & BxDFType::REFLECTION;
        }

        self.bxdfs
            .iter()
            .filter_map(|bxdf| {
                if bxdf.is_type(types) {
                    Some(bxdf.evaluate(&incident, &outgoing))
                } else {
                    None
                }
            })
            .sum()
    }

    pub fn sample(
        &self,
        normal: &Vec3,
        outgoing_world: &Vec3,
        types: BxDFType,
        sample: &Vec2,
    ) -> BxDFSample {
        let rotation = world_to_bxdf(normal);
        let outgoing = rotation * *outgoing_world;

        let bxdf = self.random_matching_bxdf(types);

        let mut sample = bxdf.sample(&outgoing, sample);
        sample.incident = rotation.reversed() * sample.incident;

        sample
    }

    pub fn pdf(
        &self,
        normal: &Vec3,
        incident_world: &Vec3,
        outgoing_world: &Vec3,
        types: BxDFType,
    ) -> f32 {
        let rotation = world_to_bxdf(normal);
        let incident = rotation * *incident_world;
        let outgoing = rotation * *outgoing_world;

        let (pdf, num) = self
            .bxdfs
            .iter()
            .filter_map(|bxdf| {
                if bxdf.is_type(types) {
                    Some(bxdf.pdf(&incident, &outgoing))
                } else {
                    None
                }
            })
            .fold((0.0, 0usize), |(a, num), b| (a + b, num + 1));

        if num > 0 {
            pdf / num as f32
        } else {
            0.0
        }
    }
}
