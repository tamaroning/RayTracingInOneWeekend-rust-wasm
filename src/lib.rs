mod utils;

use nalgebra::Vector3;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
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
        self.progress = (((y * WIDTH + x) as f32 / (WIDTH * HEIGHT) as f32) * 100.0) as u32;
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

fn draw(context: &CanvasRenderingContext2d) {
    let mut info = Info::new();

    //
    // Camera
    //
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = ASPECT_RATIO * viewport_height;
    let forcal_length: f64 = 1.0;

    let origin = Vector3::new(0., 0., 0.);
    let horizontal = Vector3::new(viewport_width, 0., 0.);
    let vertical = Vector3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0., 0., forcal_length);

    //
    // Render
    //
    context.save();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if x == 0 {
                info.update_progress(x, y);
                log!("progress y = {}, {}% completed", y, info.progress);
            }
            if x % RESOLUTION != 0 || y % RESOLUTION != 0 {
                continue;
            }

            let u = x as f64 / (WIDTH - 1) as f64;
            let v = 1.0 - (y as f64 / (HEIGHT - 1) as f64);
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&ray);
            write_color(&context, x, y, pixel_color);
        }
    }
    log!("Done!");
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(Vector3::new(0., 0., -1.), 0.5, ray) {
        return Color::new(1., 0., 0.);
    }
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
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

fn hit_sphere(center: Vector3<f64>, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - center;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.
}
