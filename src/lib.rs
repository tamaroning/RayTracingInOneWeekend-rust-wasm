mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;
const RESOLUTION: u32 = 16;

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

    draw(&context, 1);

    Ok(())
}

fn draw(context: &CanvasRenderingContext2d, vert_count: i32) {
    context.save();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if x % RESOLUTION != 0 || y % RESOLUTION != 0 {
                continue;
            }
            let r = x as f64;
            let g = y as f64;
            let b = 0.25;
            let a = 1.0;

            let px = x as f64;
            let py = y as f64;

            let color = JsValue::from_str(&format!("rgba({},{},{},{})", r, g, b, a));
            context.set_fill_style(&color);
            context.fill_rect(px, py, px + RESOLUTION as f64, py + RESOLUTION as f64);
        }
    }
}
