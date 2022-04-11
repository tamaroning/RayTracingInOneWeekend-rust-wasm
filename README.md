# rust-wasm-raytracing
Implementation of [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) in Rust and WebAssembly.
Ray tracer runs on your web browsers, using Canvas2D as a renderer.

## Screenshots
![Fuzzed metal](/screenshots/9.6.png)

## Requirement
- wasm-pack

## Build
```
npm run build
```

## Run
```
npm run serve
```

and open http://localhost:8080/ on your web browser.

Progress can be shown in DevTools→Console (Google Chrome).

## Configurations
Modulate `ASPECT_RATIO`, `WIDTH`, `RESOLUTION`, `SAMPLES_PER_PIXEL`, and `MAX_DEPTH` which are defined in src/lib.rs.
All parameters except for `RESOLUTION` are the same as are defined in [the RayTracing book](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

If you want render end soon, increase `RESOLUTION` and decrease `MAX_DEPTH`. 

## Commit History


## References
- https://rustwasm.github.io/docs/book/
- https://rustwasm.github.io/wasm-bindgen/examples/webgl.html
- https://raytracing.github.io/books/RayTracingInOneWeekend.html
