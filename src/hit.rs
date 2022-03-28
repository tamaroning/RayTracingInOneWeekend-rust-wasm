use std::rc::Rc;

use js_sys::Math::sqrt;
use nalgebra::Vector3;

use crate::material::Material;
use crate::ray::Ray;

pub struct HitRecord {
    pub p: Vector3<f64>,
    t: f64,
    pub normal: Vector3<f64>,
    // front_face := ray dot normal < 0.
    // i.e. true  => ray hits front of surface
    //      false => ray hits front of surface
    pub front_face: bool,

    pub material: Rc<dyn Material>,
}

impl HitRecord {
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3<f64>) {
        self.front_face = ray.direction.dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList<T>
where
    T: Hittable,
{
    objects: Vec<T>,
}

impl<T> HittableList<T>
where
    T: Hittable,
{
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: T) {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            // get a hit_record of the closest object by passing
            // closest_so_far as t_max
            match object.hit(ray, t_min, closest_so_far) {
                Some(hit_record) => {
                    closest_so_far = hit_record.t;
                    hit_anything = Some(hit_record);
                }
                None => (),
            }
        }
        hit_anything
    }
}

pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let half_b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return None;
        }
        let sqrtd = sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            return None;
        }

        let mut hit_record = HitRecord {
            p: ray.at(root),
            t: root,
            normal: Default::default(),
            front_face: Default::default(),
            material: Rc::clone(&self.material),
        };
        let outward_normal = (ray.at(root) - self.center) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);
        Some(hit_record)
    }
}
