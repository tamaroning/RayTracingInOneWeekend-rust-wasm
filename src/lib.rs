mod utils;
mod ray;
mod hit;

use hit::Hittable;
use nalgebra::Vector3;
use rand::prelude::ThreadRng;
use std::f64::INFINITY;
use utils::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use ray::Ray;
use hit::{HittableList, Sphere};

const ASPECT_RATIO: f64 = 16. / 9.;
const WIDTH: u32 = 512;
const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
const RESOLUTION: u32 = 1;
const SAMPLES_PER_PIXEL: u32 = 4;

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

#[allow(dead_code)]
struct Camera {
    viewport_height: f64,
    viewport_width: f64,
    forcal_length: f64,

    origin: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
}

impl Camera {
    fn new() -> Self {
        let viewport_height: f64 = 2.;
        let viewport_width: f64 = ASPECT_RATIO * viewport_height;
        let forcal_length: f64 = 1.;

        let origin = Vector3::new(0., 0., 0.);
        let horizontal = Vector3::new(viewport_width, 0., 0.);
        let vertical = Vector3::new(0., viewport_height, 0.);
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - Vector3::new(0., 0., forcal_length);

        Camera {
            viewport_height,
            viewport_width,
            forcal_length,

            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}

fn draw(context: &CanvasRenderingContext2d) {
    let mut info = Info::new();
    let mut rng = rand::thread_rng();

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
    let camera = Camera::new();

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

            let mut pixel_color = Color::new(0., 0., 0.);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u =
                    (x as f64 + random_f64(&mut rng, 0., RESOLUTION as f64)) / (WIDTH - 1) as f64;
                let v = 1.
                    - (y as f64 + random_f64(&mut rng, 0., RESOLUTION as f64))
                        / (HEIGHT - 1) as f64;

                let ray = camera.get_ray(u, v);

                pixel_color += ray_color(&ray, &world, &mut rng);
            }
            write_color(&context, x, y, pixel_color);
        }
    }
    log!("Done!");
}

fn ray_color<T>(ray: &Ray, world: &HittableList<T>, rng: &mut ThreadRng) -> Color
where
    T: Hittable,
{
    let color = match world.hit(ray, 0., INFINITY) {
        Some(hit_record) => {
            let target = hit_record.p + hit_record.normal + random_vec3_in_unit_spehere(rng);
            0.5 * ray_color(&Ray::new(hit_record.p, target - hit_record.p), world, rng)
        },
        None => {
            let unit_direction = ray.direction.normalize();
            let t = 0.5 * (unit_direction.y + 1.);
            (1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
        }
    };
    color / SAMPLES_PER_PIXEL as f64
}

fn write_color(context: &CanvasRenderingContext2d, x: u32, y: u32, color: Color) {
    let (r, g, b) = (color.x, color.y, color.z);
    let px = x as f64;
    let py = y as f64;
    let color = JsValue::from_str(&format!(
        "rgba({},{},{},{})",
        255. * r,
        255. * g,
        255. * b,
        255.
    ));
    context.set_fill_style(&color);
    context.fill_rect(px, py, px + RESOLUTION as f64, py + RESOLUTION as f64);
}
