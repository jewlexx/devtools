#[macro_export]
macro_rules! jsv {
    ($($tt:tt)*) => {
        wasm_bindgen::JsValue::from_serde(&$($tt)*)
    };
}
