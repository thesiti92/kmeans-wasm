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
pub fn run_kmeans(k: i32, points_per_cluster: i64) -> JsValue {
    set_panic_hook();

    let points = kmeans::gen_clusters(k, points_per_cluster);
    let (clusters, assignments) = kmeans::kmeans(points, k);
    JsValue::from_serde(&(clusters, assignments)).unwrap()
}
