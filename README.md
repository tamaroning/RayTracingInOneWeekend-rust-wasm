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

## References
- https://rustwasm.github.io/docs/book/
- https://rustwasm.github.io/wasm-bindgen/examples/webgl.html
- https://raytracing.github.io/books/RayTracingInOneWeekend.html
