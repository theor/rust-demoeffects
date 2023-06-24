use rillus::reflect;
use rillus::wasm_bindgen;
use wasm_bindgen::JsValue;
#[reflect]
pub fn compute(i: i32, j: i32) -> i32 {
    i * 2 + j
}
// #[wasm_bindgen]
// pub struct TestStruct {
//     pub a: i32,
//     pub k: Kind,
// }
// #[wasm_bindgen]
// impl TestStruct{
//     #[wasm_bindgen(constructor)]
//     pub fn new() -> TestStruct {
//         TestStruct { a: 4, k: Kind::B, }
//     }
// }
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum Kind {
    A,
    B,
}
#[reflect]
pub fn svg_test(i: i32, j: i32, k: Kind) -> String {
    let mut svg = String::new();
    eprintln!("i {}", i);
    svg.push_str("<svg  viewBox=\"0 0 100 100\" xmlns=\"http://www.w3.org/2000/svg\">\n");
    svg.push_str(format!("<circle r=\"10\" cx=\"{}\" cy=\"{}\"/>\n", i, j).as_str());
    svg.push_str("</svg>");
    svg
}

#[wasm_bindgen(start)]
fn main() {
    // setup();
}

#[inline]
pub fn number(base: u8, mut index: usize) -> f64 {
    let mut factor = 1.0;
    let mut result = 0.0;
    while index > 0 {
        factor /= f64::from(base);
        result += factor * (index % usize::from(base)) as f64;
        index /= usize::from(base);
    }
    result
}
#[wasm_bindgen]
pub fn halton_demo(step: usize, x: &mut [f64]) {
    let mut j = 0;
    for i in 0..x.len() {
        x[i] = number(11, j);
        j += step;
    }
}
