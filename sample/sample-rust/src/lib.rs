mod display;
mod fire;
mod utils;
mod plasma;
mod tunnel;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
fn main() {
    set_panic_hook();
}

// #[inline]
// pub fn number(base: u8, mut index: usize) -> f64 {
//     let mut factor = 1.0;
//     let mut result = 0.0;
//     while index > 0 {
//         factor /= f64::from(base);
//         result += factor * (index % usize::from(base)) as f64;
//         index /= usize::from(base);
//     }
//     result
// }
// #[wasm_bindgen]
// pub fn halton_demo(step: usize, x: &mut [f64]) {
//     let mut j = 0;
//     //    let step = 409;
//     for i in 0..x.len() / 2 {
//         x[i * 2] = number(11, j);
//         x[i * 2 + 1] = number(7, j);
//         j += step;
//     }
// }





fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    // #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
