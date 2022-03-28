use rand::prelude::ThreadRng;

use super::Color;
use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::utils::{random_unit_vector, near_zero};

pub trait Material {
    fn scatter(
        &self,
        _ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit_record.normal + random_unit_vector(rng);

        // Catch degenerate scatter direction
        if near_zero(&scatter_direction) {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray::new(hit_record.p, scatter_direction);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal {
            albedo
        }
    }
}

impl Material for Metal {
    fn scatter(
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

