use rillus::reflect;
use rillus::wasm_bindgen;
#[reflect]
pub fn compute(i: i32, j: i32) -> i32 {
    i * 2 + j
}
#[wasm_bindgen]
pub struct TestStruct {
    pub a: i32,
    pub k: Kind,
}
#[wasm_bindgen]
impl TestStruct{
    #[wasm_bindgen(constructor)]
    pub fn new() -> TestStruct {
        TestStruct { a: 4, k: Kind::B, }
    }
}
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum Kind { A, B }
#[reflect]
pub fn svg_test(i: i32, j: i32, k: Kind) -> String {
    let mut svg = String::new();
    eprintln!("i {}", i);
    svg.push_str("<svg  viewBox=\"0 0 100 100\" xmlns=\"http://www.w3.org/2000/svg\">");
    svg.push_str(format!("<circle r=\"10\" cx=\"{}\" cy=\"{}\"/>", i, j).as_str());
    svg.push_str("</svg>");
    svg
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"

export const svg_func = { desc: svg_test_desc, func: svg_test, };

"#;