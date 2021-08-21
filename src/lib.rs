use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::console;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(target_arch = "wasm32")]
fn log(string: &str) {
    console::log_1(&JsValue::from_str(string));
}

#[cfg(not(target_arch = "wasm32"))]
fn log(string: &str) {
    println!("{}", string);
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn run() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(feature = "panic_hook")]
    console_error_panic_hook::set_once();


    // Your code goes here!
    log("Hello world!");
}
