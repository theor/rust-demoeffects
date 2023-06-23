mod utils;

pub use rillus_macros::reflect;
pub use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
pub use serde_wasm_bindgen::to_value;
use serde::{Serialize, Deserialize};
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[derive(Serialize, Deserialize)]
pub struct Param<'a> {
    pub value_type: &'a str,
    pub name: &'a str,
}
#[derive(Serialize, Deserialize)]
pub struct Function<'a> {
    #[serde(borrow)]
    pub return_type: Param<'a>,
    pub params: Vec<Param<'a>>,
}
