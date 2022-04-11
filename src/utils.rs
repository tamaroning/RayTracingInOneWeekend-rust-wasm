use std::f64::consts::PI;

use js_sys::Math::{cos, sin, sqrt};
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

// different diffuse formulation
// 1
pub fn random_vec3_in_unit_spehere(rng: &mut ThreadRng) -> Vector3<f64> {
    loop {
        let v = random_vec3(rng);
        if sqnorm(v) < 1. {
            return v;
        }
    }
}

// 2
pub fn random_unit_vector(rng: &mut ThreadRng) -> Vector3<f64> {
    let a = random_f64(rng, 0., 2. * PI);
    let z = random_f64(rng, -1., 1.);
    let r = sqrt(1. - z * z);
    Vector3::new(r * cos(a), r * sin(a), z)
}

pub fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - 2. * v.dot(n) * n
}

pub fn near_zero(v: &Vector3<f64>) -> bool {
    let s = 1e-8;
    (v.x < s) && (v.y < s) && (v.z < s)
}

pub fn refract(uv: &Vector3<f64>, n: &Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let cos_theta = (-uv.dot(n)).min(1.);
    let r_out_parallel = etai_over_etat * (uv + cos_theta * n);
    let r_out_perp = -sqrt((1.0 - sqnorm(r_out_parallel)).abs()) * n;
    r_out_parallel + r_out_perp
}

pub fn sqnorm(v: Vector3<f64>) -> f64 {
    v.x * v.x + v.y * v.y + v.z + v.z
}

pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powf(5.)
}

pub fn deg_to_rad(deg: f64) -> f64 {
    deg / 360. * 2. * PI
}

pub fn rad_to_deg(rad: f64) -> f64 {
    rad * 180. / PI
}

pub fn random_in_unit_disk(rng: &mut ThreadRng) -> Vector3<f64> {
    loop {
        let p = Vector3::new(random_f64(rng, -1., 1.), random_f64(rng, -1., 1.), 0.);
        if sqnorm(p) < 1. {
            return p;
        }
    }
}
