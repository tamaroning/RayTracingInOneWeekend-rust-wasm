mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

/*
#[derive(Debug, Clone, Copy)]
struct Color(f32, f32, f32, f32);
struct Vec3(f32, f32, f32);
*/

fn main() {
    #[wasm_bindgen(start)]
    pub fn start() -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
        canvas.set_height(HEIGHT);
        canvas.set_width(WIDTH);

        let context = canvas
            .get_context("webgl2")?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;

        // FIXME: Is gl_PointSize OK?
        let vert_shader = compile_shader(
            &context,
            WebGl2RenderingContext::VERTEX_SHADER,
            r##"#
        attribute vec4 position;
        attribute vec4 color;

        varying lowp vec4 vColor;
        void main(void) {
            gl_Position = position;
            vColor = color;
        }
        "##,
        )?;

        let frag_shader = compile_shader(
            &context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            r##"#
        
        varying lowp vec4 vColor;
        
        void main(void) {
            gl_FragColor = vColor;
        }
        "##,
        )?;

        let program = link_program(&context, &vert_shader, &frag_shader)?;
        context.use_program(Some(&program));

        //
        // Computation Start
        //
        /*
        let mut vertices: [f32;(WIDTH * HEIGHT * 3) as usize] = [0.0; (WIDTH * HEIGHT * 3) as usize];
        let mut colors: [f32; (WIDTH * HEIGHT * 4) as usize] = [0.0; (WIDTH * HEIGHT * 4) as usize];
        for y in 0..HEIGHT {
            for x in 0.. WIDTH {
                let px = (x / (WIDTH-1)) as f32 * 2.0 - 1.0;
                let py = (y / (HEIGHT-1)) as f32 * 2.0 - 1.0;
                vertices[((y * WIDTH + x) * 3) as usize] = px;
                vertices[((y * WIDTH + x) * 3) as usize + 1] = py;
                // z is already set to 0.

                let r = (x / (WIDTH-1)) as f32;
                let g = (y / (HEIGHT-1)) as f32;
                let b = 0.25;
                let a = 0.5;
                colors[((y * WIDTH + x) * 4) as usize] = r;
                colors[((y * WIDTH + x) * 4) as usize + 1] = g;
                colors[((y * WIDTH + x) * 4) as usize + 2] = b;
                colors[((y * WIDTH + x) * 4) as usize + 3] = a;
            }
        }
        log!("{:?}", &vertices[0..100]);
        */
        let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
        let colors: [f32; 12] = [1.0, 1.0, 0.0, 0.5, 1.0, 1.0, 0.0, 0.5, 1.0, 1.0, 0.0, 0.5];

        let position_attribute_location = context.get_attrib_location(&program, "position");
        let position_buffer = context.create_buffer().ok_or("Failed to create buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&position_buffer));

        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        let vao = context
            .create_vertex_array()
            .ok_or("Could not create vertex array object")?;
        context.bind_vertex_array(Some(&vao));
        context.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
        context.enable_vertex_attrib_array(position_attribute_location as u32);
        context.bind_vertex_array(Some(&vao));

        /* 
        let color_attribute_location = context.get_attrib_location(&program, "color");
        let color_buffer = context.create_buffer().ok_or("Failed to create buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&color_buffer));
        unsafe {
            let colors_array_buf_view = js_sys::Float32Array::view(&colors);
            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &colors_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
        let vao = context
            .create_vertex_array()
            .ok_or("Could not create vertex array object")?;
        context.bind_vertex_array(Some(&vao));
        context.vertex_attrib_pointer_with_i32(0, 4, WebGl2RenderingContext::FLOAT, false, 0, 0);
        context.enable_vertex_attrib_array(color_attribute_location as u32);
        context.bind_vertex_array(Some(&vao));
        */

        let vert_count = (vertices.len() / 3) as i32;
        //
        // Computation End
        //

        draw(&context, vert_count);

        Ok(())
    }

    fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
        context.clear_color(0.0, 0.0, 1.0, 0.5);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
    }

    pub fn compile_shader(
        context: &WebGl2RenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }

    pub fn link_program(
        context: &WebGl2RenderingContext,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        context.attach_shader(&program, vert_shader);
        context.attach_shader(&program, frag_shader);
        context.link_program(&program);

        if context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }
}
