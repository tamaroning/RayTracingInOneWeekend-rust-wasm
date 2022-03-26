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