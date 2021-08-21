#[macro_use]
extern crate log;

use wasm_bindgen::prelude::*;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn run() {
    // Setup the WASM logger.
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(Default::default());

    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(feature = "panic_hook")]
    console_error_panic_hook::set_once();


    // Your code goes here!
    info!("Hello World!");
}
