// Issue: https://github.com/leptos-rs/leptos/issues/3433
#![recursion_limit = "512"]

pub mod app;
pub mod core;
pub mod landing_page;
pub mod project;
pub mod shared;
pub mod system;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
