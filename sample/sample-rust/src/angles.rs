use line_drawing::{BresenhamCircle, Bresenham};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn draw_angles(buffer: &mut [u32],w:i32, a: f32, r: f32){
    buffer.fill(0x77777777);
    let c = BresenhamCircle::new(w / 2, w / 2, (w as f32/2.0 * r) as i32);
    for (x,y) in c {
buffer[x as usize + y as usize * w as usize] = 0xFFFFFFFF;
    }
    let center = (w) / 2;
let t = (
    (center + (w as f32 * r * 0.5 * a.cos()) as i32).max(0).min(w as i32 - 1),
    (center + (w as f32 * r * 0.5 * a.sin()) as i32).max(0).min(w as i32 - 1)
);
for(x, y) in Bresenham::new((center, center), t) {
    buffer[x as usize + y as usize * w as usize] = 0xFF00FFFF;

}
}