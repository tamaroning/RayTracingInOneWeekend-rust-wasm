use rand::prelude::ThreadRng;

use super::Color;
use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::utils::random_unit_vector;

pub trait Material {
    fn scatter(
        ray: &Ray,
        hit_record: HitRecord,
        attenuation: Color,
        scattered: &Ray,
        rng: &mut ThreadRng,
    );
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Lambertian {
    pub fn scatter(
        &self,
        _ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Color)> {
        let scattered_direction = hit_record.normal + random_unit_vector(rng);
        let scattered = Ray::new(hit_record.p, scattered_direction);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}
