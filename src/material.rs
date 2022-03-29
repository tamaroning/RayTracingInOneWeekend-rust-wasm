use js_sys::Math::sqrt;
use rand::prelude::ThreadRng;

use super::Color;
use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::utils::*;

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
        let scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * random_unit_vector(rng),
        );
        if scattered.direction.dot(&hit_record.normal) > 0. {
            let attenuation = self.albedo;
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectic {
    refraction_idx: f64,
}

impl Dielectic {
    pub fn new(ref_idx: f64) -> Self {
        Dielectic {
            refraction_idx: ref_idx,
        }
    }
}

impl Material for Dielectic {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1., 1., 1.);
        let refraction_ratio = if hit_record.front_face {
            1. / self.refraction_idx
        } else {
            self.refraction_idx
        };

        let unit_direction = ray_in.direction.normalize();
        let cos_theta = (-unit_direction.dot(&hit_record.normal)).min(1.);
        let sin_theta = sqrt(1. - cos_theta * cos_theta);

        let cannot_reflact = refraction_ratio * sin_theta > 1.;
        let direction = if cannot_reflact
            || reflectance(cos_theta, refraction_ratio) > random_f64(rng, 0., 1.)
        {
            reflect(&unit_direction, &hit_record.normal)
        } else {
            refract(&unit_direction, &hit_record.normal, refraction_ratio)
        };

        let scattered = Ray::new(hit_record.p, direction);
        Some((scattered, attenuation))
    }
}
