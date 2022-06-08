use perseus::{
    Html, PerseusApp,
    Plugins,
    // Template,
};
// use sycamore::view;
use perseus_size_opt::{perseus_size_opt, SizeOpts};

mod templates;
mod error_pages;

#[perseus::main]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(templates::index::get_template)
        .template(templates::about::get_template)
        .error_pages(error_pages::get_error_pages)
        // .template(|| {
        // Template::new("index").template(|_| {
        //     view! {
        //         p { "Hello World!" }
        //     }
        // })
        .plugins(Plugins::new()
            .plugin(
                perseus_size_opt,
                SizeOpts::default()
            ))
}

// #![recursion_limit = "512"]

// mod app;
// mod cle;

// use wasm_bindgen::prelude::*;

// // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// // allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[cfg(feature = "console_log")]
// fn init_log() {
//     use log::Level;
//     console_log::init_with_level(Level::Trace).expect("error initializing log");
// }

// // This is the entry point for the web app
// #[wasm_bindgen]
// pub fn run_cle() -> Result<(), JsValue> {
//     #[cfg(feature = "console_log")]
//     init_log();
//     // wasm_logger::init(wasm_logger::Config::default());
//     let app = yew::App::<app::App>::new();
//     app.mount(web_sys::window().unwrap().document().unwrap().query_selector("#creole-live-editor").unwrap().unwrap());
//     yew::run_loop();
//     // yew::start_app::<app::App>();
//     Ok(())
// }

// pub use crate::cle::CreoleLiveEditor;

// pub mod prelude {
//     pub use crate::CreoleLiveEditor;
// }
