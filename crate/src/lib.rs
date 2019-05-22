#[macro_use]
extern crate cfg_if;
extern crate rand;

extern crate serde;
extern crate serde_json;
extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
mod kmeans;
mod point;

use point::Point;
use wasm_bindgen::JsValue;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn run_kmeans(points: &JsValue, k: i32) -> JsValue {
    set_panic_hook();
    let points: Vec<Point> = points.into_serde().unwrap();
    // let points = vec![
    //     Point { x: 1f32, y: 1f32 },
    //     Point { x: 1f32, y: 2f32 },
    //     Point { x: 2f32, y: 1f32 },
    //     Point { x: 9f32, y: 9f32 },
    //     Point { x: 10f32, y: 10f32 },
    //     Point { x: 11f32, y: 10f32 },
    // ];
    // let k = 2;
    let (clusters, assignments) = kmeans::kmeans(points, k);
    JsValue::from_serde(&clusters).unwrap()
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // If the `console_error_panic_hook` feature is enabled this will set a panic hook, otherwise
    // it will do nothing.
    set_panic_hook();

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // let val = document.create_element("p")?;
    // val.set_inner_html(format!("{:?}", clusters).as_str());
    // body.append_child(&val)?;

    Ok(())
}
