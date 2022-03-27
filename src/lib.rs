mod utils;

use std::f64::INFINITY;

use js_sys::Math::sqrt;
use nalgebra::Vector3;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

const ASPECT_RATIO: f64 = 16. / 9.;
const WIDTH: u32 = 512;
const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
const RESOLUTION: u32 = 1;

// (r, g, b) = (x, y, z)
type Color = Vector3<f64>;

pub struct Info {
    progress: u32,
}

impl Info {
    pub fn new() -> Self {
        Info { progress: 0 }
    }

    pub fn update_progress(&mut self, x: u32, y: u32) {
        self.progress = (((y * WIDTH + x) as f32 / (WIDTH * HEIGHT) as f32) * 100.) as u32;
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.set_height(HEIGHT);
    canvas.set_width(WIDTH);

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    draw(&context);

    Ok(())
}

struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        Ray { origin, direction }
    }
    fn at(&self, t: f64) -> Vector3<f64> {
        self.origin + t * self.direction
    }
}

#[derive(Default)]
struct HitRecord {
    p: Vector3<f64>,
    t: f64,
    normal: Vector3<f64>,
    // front_face := ray dot normal < 0.
    // i.e. true  => ray hits front of surface
    //      false => ray hits front of surface
    front_face: bool,
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

trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

struct HittableList<T>
where
    T: Hittable,
{
    objects: Vec<T>,
}

impl<T> HittableList<T>
where
    T: Hittable,
{
    fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    fn add(&mut self, object: T) {
        self.objects.push(object);
    }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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

struct Sphere {
    center: Vector3<f64>,
    radius: f64,
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
            ..Default::default()
        };
        let outward_normal = (ray.at(root) - self.center) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);
        Some(hit_record)
    }
}

fn draw(context: &CanvasRenderingContext2d) {
    let mut info = Info::new();

    //
    // World
    //
    let mut world = HittableList::new();
    world.add(Sphere {
        center: Vector3::new(0., 0., -1.),
        radius: 0.5,
    });
    world.add(Sphere {
        center: Vector3::new(0., -100.5, -1.),
        radius: 100.,
    });

    //
    // Camera
    //
    let viewport_height: f64 = 2.;
    let viewport_width: f64 = ASPECT_RATIO * viewport_height;
    let forcal_length: f64 = 1.;

    let origin = Vector3::new(0., 0., 0.);
    let horizontal = Vector3::new(viewport_width, 0., 0.);
    let vertical = Vector3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vector3::new(0., 0., forcal_length);

    //
    // Render
    //
    context.save();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if x == 0 && y % 3 == 0 {
                info.update_progress(x, y);
                log!("progress y = {}, {}% completed", y, info.progress);
            }
            if x % RESOLUTION != 0 || y % RESOLUTION != 0 {
                continue;
            }

            let u = x as f64 / (WIDTH - 1) as f64;
            let v = 1. - (y as f64 / (HEIGHT - 1) as f64);

            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel_color = ray_color(&ray, &world);
            write_color(&context, x, y, pixel_color);
        }
    }
    log!("Done!");
}

fn ray_color<T>(ray: &Ray, world: &HittableList<T>) -> Color
where
    T: Hittable,
{
    if let Some(hit_record) = world.hit(ray, 0., INFINITY) {
        return 0.5 * (hit_record.normal + Vector3::new(1., 1., 1.));
    }
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.);
    (1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
}

fn write_color(context: &CanvasRenderingContext2d, x: u32, y: u32, color: Color) {
    // a is hardcoded to 0
    let (r, g, b, a) = (color.x, color.y, color.z, 1.0);
    let px = x as f64;
    let py = y as f64;
    let color = JsValue::from_str(&format!(
        "rgba({},{},{},{})",
        255. * r,
        255. * g,
        255. * b,
        255. * a
    ));
    context.set_fill_style(&color);
    context.fill_rect(px, py, px + RESOLUTION as f64, py + RESOLUTION as f64);
}
