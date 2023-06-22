mod utils;

use rillus_macros::reflect;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[derive(Serialize, Deserialize)]
pub struct Param {
    pub value_type: &'static str,
    pub name: &'static str,
}
#[derive(Serialize, Deserialize)]
pub struct Function {
    pub return_type: &'static str,
    pub params: Vec<Param>,
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

// #[wasm_bindgen]
// pub fn compute_descriptor() -> JsValue {
//     serde_wasm_bindgen::to_value(&Function {
//         return_type: "number",
//         params: vec![Param { name:"i", value_type: "number"}]
//     }).unwrap()
// }
// #[wasm_bindgen]
#[reflect]
pub fn compute(i:i32, j:i32) -> i32 {
 i*2+j
}