use wasm_bindgen::prelude::*;
use web_sys::ServiceWorker;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    // wasm_logger::init(wasm_logger::Config::default());
    let sw = ;//ServiceWorker::
    sw.add_event_listener_with_callback("install", |e : ex| {
      e.wait_until()
    });
    Ok(())
}