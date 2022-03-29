use crate::{ray::Ray, utils::*};
use js_sys::Math::tan;
use nalgebra::Vector3;

pub struct Camera {
    origin: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
}

impl Camera {
    // See https://raytracing.github.io/books/RayTracingInOneWeekend.html#positionablecamera/positioningandorientingthecamera
    pub fn new(
        lookfrom: Vector3<f64>,
        lookat: Vector3<f64>,
        vup: Vector3<f64>,
        vfov: f64, /* vertical field-of-view in degrees */
        aspect_ratio: f64,
    ) -> Self {
        let theta = deg_to_rad(vfov);
        let h = tan(theta / 2.);
        let viewport_height: f64 = 2. * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: lookfrom - horizontal / 2. - vertical / 2. - w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
