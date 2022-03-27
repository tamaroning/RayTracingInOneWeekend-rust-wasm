use std::f64::consts::PI;

use js_sys::Math::{sqrt, cos, sin};
use nalgebra::Vector3;
use rand::{prelude::ThreadRng, Rng};

// This macro is retrived from https://github.com/lykhouzov/rust-wasm-webgl/blob/master/src/utils.rs
#[macro_export]
macro_rules! float_32_array {
    ($arr:expr) => {{
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();
        let arr_location = $arr.as_ptr() as u32 / 4;
        let array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(arr_location, arr_location + $arr.len() as u32);
        array
    }};
}

// This macro is retrived from https://github.com/lykhouzov/rust-wasm-webgl/blob/master/src/utils.rs
#[macro_export]
macro_rules! uint_16_array {
    ($arr:expr) => {{
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();
        let arr_location = $arr.as_ptr() as u32 / 2;
        let array = js_sys::Uint16Array::new(&memory_buffer)
            .subarray(arr_location, arr_location + $arr.len() as u32);
        array
    }};
}

// This macro is retrived from https://github.com/lykhouzov/rust-wasm-webgl/blob/master/src/utils.rs
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn random_f64(rng: &mut ThreadRng, min: f64, max: f64) -> f64 {
    rng.gen::<f64>() * (max - min) + min
}

pub fn random_vec3(rng: &mut ThreadRng) -> Vector3<f64> {
    Vector3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
}

pub fn random_vec3_in_unit_spehere(rng: &mut ThreadRng) -> Vector3<f64> {
    loop {
        let v = random_vec3(rng);
        // FIXME: use sqnorm
        if v.norm() < 1. {
            return v;
        }
    }
}

pub fn random_unit_vector(rng: &mut ThreadRng) -> Vector3<f64> {
    let a = random_f64(rng, 0., 2. * PI);
    let z = random_f64(rng, - 1., 1.);
    let r = sqrt(1. - z * z);
    Vector3::new(r * cos(a), r * sin(a), z)
}