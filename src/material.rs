use rand::prelude::ThreadRng;

use super::Color;
use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::utils::{near_zero, random_unit_vector, reflect};

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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Color)> {
        let reflected = reflect(&ray_in.direction.normalize(), &hit_record.normal);
        let scattered = Ray::new(hit_record.p, reflected + self.fuzz * random_unit_vector(rng));
        if scattered.direction.dot(&hit_record.normal) > 0. {
            let attenuation = self.albedo;
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
