mod utils;

use nalgebra::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;
const RESOLUTION: u32 = 16;

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

struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
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

            let color = Color {
                r: x as f64,
                g: y as f64,
                b: 0.25,
                a: 1.0,
            };
            write_color(&context, x, y, color);
        }
    }
    log!("Done!");
}

fn write_color(context: &CanvasRenderingContext2d, x: u32, y: u32, color: Color) {
    let Color { r, g, b, a } = color;
    let px = x as f64;
    let py = y as f64;
    let color = JsValue::from_str(&format!("rgba({},{},{},{})", r, g, b, a));
    context.set_fill_style(&color);
    context.fill_rect(px, py, px + RESOLUTION as f64, py + RESOLUTION as f64);
}
