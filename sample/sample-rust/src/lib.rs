use colorsys::Hsl;
use colorsys::HslRatio;
use colorsys::Rgb;
use rillus::reflect;
use rillus::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::Element;
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
    //    let step = 409;
    for i in 0..x.len() / 2 {
        x[i * 2] = number(11, j);
        x[i * 2 + 1] = number(7, j);
        j += step;
    }
}

fn set<'a, T>(b: &mut [u8], i: usize, c: &'a T)
where
    Rgb: From<&'a T>,
{
    let rgb: Rgb = Rgb::from(c);
    let bytes: [u8; 3] = rgb.into();
    // web_sys::console::log_1(&JsValue::from_f64(bytes[0] as f64));
    b[i] = bytes[0];
    b[i + 1] = bytes[1];
    b[i + 2] = bytes[2];
    b[i + 3] = 255;
}
fn to_byte(x: f32) -> u8 {
    (x * 255.0).clamp(0.0, 255.0) as u8
}

static mut I: i32 = 0;
#[wasm_bindgen]
pub fn render(t: f32, b: &mut [u8], fire: &mut [u8], w: usize, h: usize) {
    // let (cx," cy) = (w as i32 / 2 + step, h  as i32 / 2 + step);
    let mut palette: Vec<Hsl> = Vec::with_capacity(256);
    for i in 0..256 {
        let fy = i as f32 / 255.0;
        // let h = fy / 3.0;
        let h = (187.0 +  (235.0 - 187.0) * fy) / 360.0;
        palette.push(HslRatio::from((h, 1.0, 1.0f32.min(fy * 2.0))).into());
    }
    // random bottom row
    for x in 0..w {
        let fi = (h - 1) * w + x;
        let i = (h-1) * w * 4 + x as usize * 4;
        fire[fi] = rand::random();
        set(b, i, &palette[fire[fi as usize] as usize]);
    }

    for y in 0..h - 1 {
        let fy = y as f32 / h as f32;

        for x in 0..w {
            let fi = (y * w + x) as usize;

            fire[fi] = (((fire[((y + 1) % h) * w + (x - 1 + w) % w] as u32
                + fire[((y + 1) % h) * w + x] as u32
                + fire[((y + 1) % h) * w + (x + 1) % w] as u32
                + fire[((y + 2) % h) * w + x] as u32) as f32
                * t)
                / 4.0) as u8;
            // fire[fi] = (fy * 255.0) as u8;
            let fx = x as f32 / w as f32;
            let i = y as usize * w * 4 + x as usize * 4;

            // let c: &Hsl = &palette[((1.0 - fy) * 255.0) as usize];// colorsys::HslRatio::from((fx, fy, 0.5)).into();
            let c: &Hsl = &palette[fire[fi] as usize]; // colorsys::HslRatio::from((fx, fy, 0.5)).into();
            set(b, i, c);
        }
    }

    // let pxCount = x.len() / 4;
    // for i in 0..pxCount {
    //     let r = (i * 255 / pxCount).clamp(0, 255) as u8;
    //     let o =i * 4;
    //     x[o] = r;
    //     x[o+1] = (r as i32 + step).clamp(0, 255) as u8;
    //     x[o+2] = (r as i32 - step).clamp(0, 255) as u8;
    //     x[o+3] = 255;
    // }
}

#[wasm_bindgen]
pub fn make_fragment(x: f32, t: f32) -> Element {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    // let f = web_sys::DocumentFragment::new().unwrap();
    let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg").unwrap()  
    // .dyn_into::<web_sys::SvgElement>().unwrap();
    ;
    // svg.set_attribute("xmlns", "http://www.w3.org/2000/svg");
    svg.set_attribute("viewBox", "0 0 100 100");
    svg.set_attribute("width", "100");
    svg.set_attribute("height", "100");

    // let circle = document.create_element("circle").unwrap();
    let circle = document
        .create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")
        .unwrap();
    circle.set_attribute("r", "10");
    circle.set_attribute("cx", "50");
    circle.set_attribute("cy", &format!("{}", x + t.sin() * 20.0));
    svg.append_child(&circle);

    svg
    // let p = document.create_element("p").unwrap();
    // p.set_text_content(Some(&format!("asd {}", x)));
    // f.append_child(&svg).unwrap();
    // f
}
