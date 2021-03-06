mod camera;
mod hit;
mod material;
mod ray;
mod utils;

use camera::Camera;
use hit::Hittable;
use hit::{HittableList, Sphere};
use js_sys::Math::{atan, sqrt};
use nalgebra::Vector3;
use rand::prelude::ThreadRng;
use rand::Rng;
use ray::Ray;
use std::f64::INFINITY;
use std::rc::Rc;
use utils::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

use crate::material::{Dielectic, Lambertian, Metal};

const ASPECT_RATIO: f64 = 3. / 2.;
const WIDTH: u32 = 1200;
const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
const RESOLUTION: u32 = 1;
const SAMPLES_PER_PIXEL: u32 = 8;
const MAX_DEPTH: i32 = 10;

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

fn draw(context: &CanvasRenderingContext2d) {
    let mut info = Info::new();
    let mut rng = rand::thread_rng();

    //
    // World
    //
    // Replace this with image15_scene(), image20_scene(), or image21_scene(&mut rng).
    // this number of image corresponds to the book:
    // https://raytracing.github.io/books/RayTracingInOneWeekend.html
    let (world, camera) = image21_scene(&mut rng);

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

                let ray = camera.get_ray(u, v, &mut rng);

                pixel_color += ray_color(&ray, &world, &mut rng, MAX_DEPTH);
            }
            write_color(&context, x, y, pixel_color);
        }
    }
    log!("Done!");
}

fn ray_color<T>(ray: &Ray, world: &HittableList<T>, rng: &mut ThreadRng, depth: i32) -> Color
where
    T: Hittable,
{
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth < 0 {
        return Color::new(0., 0., 0.);
    }
    match world.hit(ray, 0.001, INFINITY) {
        Some(hit_record) => {
            match hit_record.material.scatter(ray, &hit_record, rng) {
                Some((scattered, attenuation)) => {
                    // FIXME: in place
                    return attenuation.component_mul(&ray_color(&scattered, world, rng, depth));
                }
                None => {
                    return Color::new(0., 0., 0.);
                }
            }
        }
        None => {
            let unit_direction = ray.direction.normalize();
            let t = 0.5 * (unit_direction.y + 1.);
            (1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
        }
    }
}

fn write_color(context: &CanvasRenderingContext2d, x: u32, y: u32, color: Color) {
    let (r, g, b) = (color.x, color.y, color.z);

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1. / SAMPLES_PER_PIXEL as f64;
    let r = sqrt(scale * r);
    let g = sqrt(scale * g);
    let b = sqrt(scale * b);

    let px = x as f64;
    let py = y as f64;
    let color = JsValue::from_str(&format!(
        "rgba({},{},{},{})",
        255. * clamp(r, 0., 0.999),
        255. * clamp(g, 0., 0.999),
        255. * clamp(b, 0., 0.999),
        255.
    ));
    context.set_fill_style(&color);
    context.fill_rect(px, py, px + RESOLUTION as f64, py + RESOLUTION as f64);
}

fn image15_scene() -> (HittableList<Sphere>, Camera) {
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectic::new(1.5); //Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.);

    world.add(Sphere {
        center: Vector3::new(0., -100.5, -1.),
        radius: 100.,
        material: Rc::new(material_ground),
    });
    world.add(Sphere {
        center: Vector3::new(0., 0., -1.),
        radius: 0.5,
        material: Rc::new(material_center),
    });
    world.add(Sphere {
        center: Vector3::new(-1., 0., -1.),
        radius: 0.5,
        material: Rc::new(material_left),
    });
    world.add(Sphere {
        center: Vector3::new(1., 0., -1.),
        radius: 0.5,
        material: Rc::new(material_right),
    });

    let lookfrom = Vector3::new(0., 0., 0.);
    let lookat = Vector3::new(0., 0., -1.);
    let vup = Vector3::new(0., 1., 0.);
    let vfov = rad_to_deg(2. * atan(2.0 /* = h */ / 2.));
    let dist_to_focus = (lookfrom - lookat).norm();
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );
    (world, camera)
}

fn image20_scene() -> (HittableList<Sphere>, Camera) {
    //
    // World
    //
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectic::new(1.5); //Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.);

    world.add(Sphere {
        center: Vector3::new(0., -100.5, -1.),
        radius: 100.,
        material: Rc::new(material_ground),
    });
    world.add(Sphere {
        center: Vector3::new(0., 0., -1.),
        radius: 0.5,
        material: Rc::new(material_center),
    });
    {
        world.add(Sphere {
            center: Vector3::new(-1., 0., -1.),
            radius: 0.5,
            material: Rc::new(material_left.clone()),
        });
        world.add(Sphere {
            center: Vector3::new(-1., 0., -1.),
            radius: -0.45,
            material: Rc::new(material_left),
        });
    }
    world.add(Sphere {
        center: Vector3::new(1., 0., -1.),
        radius: 0.5,
        material: Rc::new(material_right),
    });

    //
    // Camera
    //
    let lookfrom = Vector3::new(3., 3., 2.);
    let lookat = Vector3::new(0., 0., -1.);
    let vup = Vector3::new(0., 1., 0.);
    let dist_to_focus = (lookfrom - lookat).norm();
    let aperture = 2.;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );
    (world, camera)
}

fn image21_scene(rng: &mut ThreadRng) -> (HittableList<Sphere>, Camera) {
    //
    // World
    //
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere {
        center: Vector3::new(0., -1000., 0.),
        radius: 1000.,
        material: Rc::new(ground_material),
    });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vector3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Vector3::new(4., 0.2, 0.)).norm() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo: Color = random_vec3(rng).component_mul(&random_vec3(rng));
                    let material = Lambertian::new(albedo);
                    world.add(Sphere {
                        center,
                        radius: 0.2,
                        material: Rc::new(material),
                    })
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_vec3(rng) * 0.5 + Vector3::new(0.5, 0.5, 0.5);
                    let fuzz = random_f64(rng, 0., 0.5);
                    let material = Metal::new(albedo, fuzz);
                    world.add(Sphere {
                        center,
                        radius: 0.2,
                        material: Rc::new(material),
                    });
                } else {
                    let material = Dielectic::new(1.5);
                    world.add(Sphere {
                        center,
                        radius: 0.2,
                        material: Rc::new(material),
                    });
                }
            }
        }
    }

    let material1 = Dielectic::new(1.5);
    world.add(Sphere {
        center: Vector3::new(0., 1., 0.),
        radius: 1.,
        material: Rc::new(material1),
    });

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere {
        center: Vector3::new(-4., 1., 0.),
        radius: 1.,
        material: Rc::new(material2),
    });

    //let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.);
    //world.add(Sphere { center: Vector3::new(4., 1., 0.), radius: 1., material: Rc::new(material3) });

    //
    // Camera
    //
    let lookfrom = Vector3::new(13., 2., 3.);
    let lookat = Vector3::new(0., 0., 0.);
    let vup = Vector3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );
    (world, camera)
}
