#![recursion_limit = "512"]

mod app;
mod cle;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(feature = "console_log")]
fn init_log() {
    use log::Level;
    console_log::init_with_level(Level::Trace).expect("error initializing log");
}

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    #[cfg(feature = "console_log")]
    init_log();
    // wasm_logger::init(wasm_logger::Config::default());
    let app = yew::App::<app::App>::new();
    app.mount(web_sys::window().unwrap().document().unwrap().query_selector("#creole-live-editor").unwrap().unwrap());
    yew::run_loop();
    // yew::start_app::<app::App>();
    Ok(())
}

pub use crate::cle::CreoleLiveEditor;

pub mod prelude {
    pub use crate::CreoleLiveEditor;
}
