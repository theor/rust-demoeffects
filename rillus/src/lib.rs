mod utils;

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[derive(Serialize, Deserialize)]
pub struct Param {
    pub value_type: String,
    pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct Function {
    pub return_type: String,
    pub params: Vec<Param>,
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn compute_descriptor() -> JsValue {
    serde_wasm_bindgen::to_value(&Function {
        return_type: "number".to_string(),
        params: vec![Param { name:"i".to_string(), value_type: "number".to_string()}]
    }).unwrap()
}
#[wasm_bindgen]
pub fn compute(i:i32) -> i32 {
 i*2
}