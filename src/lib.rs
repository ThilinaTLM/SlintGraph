#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
fn run() {
    bare_bones();
    using_a_macro();
    using_web_sys();
}

// Binding `console.log` manually, without `web_sys`.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    // Binding `console.log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // Polymorphic binding for `console.log` with `u32`
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments with `console.log`
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[cfg(target_arch = "wasm32")]
fn bare_bones() {
    log("Hello from Rust!");
    log_u32(42);
    log_many("Logging", "many values!");
}

// Macro for `console.log` functionality similar to `println!`
#[cfg(target_arch = "wasm32")]
macro_rules! console_log {
    // This uses the `log` function defined in `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[cfg(target_arch = "wasm32")]
fn using_a_macro() {
    console_log!("Hello {}!", "world");
    console_log!("Let's print some numbers...");
    console_log!("1 + 3 = {}", 1 + 3);
}

// Using `web_sys` crate for `console.log`
#[cfg(target_arch = "wasm32")]
fn using_web_sys() {
    use web_sys::{console, JsValue};

    console::log_1(&"Hello using web-sys".into());

    let js: JsValue = 4.into();
    console::log_2(&"Logging arbitrary values looks like".into(), &js);
}
