# [WIP] Emscripten-rs

Working with Emscripten in Rust.


## Details

I tried to spawn a Web Worker thread from Emscripten but it didn't work for some reason. I'm not sure whether the problem is the unsafe part of the application or the way I call the work.  
Although it does load the `worker.js` in the browser (compiled worker file) but it doesn't call the function inside `worker.rs`.

I posted a question here: https://stackoverflow.com/questions/45248078/creating-web-worker-from-rust-with-emscripten-target

*I took some parts of the code from different repos on Github and Gitlab, but the other repos were MIT-licensed as well.*

## Build

`cargo build --target=wasm32-unknown-emscripten --verbose`

## License

MIT
