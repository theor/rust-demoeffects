use bevy::math::{vec2, vec3};

use wasm_bindgen::prelude::wasm_bindgen;

use colorsys::{Hsl, Rgb};

use crate::utils::lerp;

#[wasm_bindgen]
pub struct Plasma {
    w: usize,
    h: usize,
    palette: Vec<u32>,
}

fn col32(r: u8, g: u8, b: u8) -> u32 {
    255 << 24 | (b as u32) << 16 | (g as u32) << 8 | (r as u32)
}
fn col32f(r: f32, g: f32, b: f32) -> u32 {
    255 << 24 | ((b * 255.0).clamp(0.0, 255.0) as u32) << 16 | ((g * 255.0).clamp(0.0, 255.0) as u32) << 8 | ((r * 255.0).clamp(0.0, 255.0) as u32)
}

#[wasm_bindgen]
impl Plasma {
    #[wasm_bindgen(constructor)]
    pub fn new(w: usize, h: usize) -> Self {
        let mut palette: Vec<u32> = vec![0; 256];
        for i in 0..palette.len() {
            let f = i as f32 / palette.len() as f32;
            let c = Rgb::from(Hsl::from((
                lerp(203.0..=31.0, f),
                82.0,
                ((f+0.5).powi(2)*100.0).clamp(10.0, 60.0)
            )));
            palette[i] = col32(
                c.red().min(255.0) as u8,
                c.green().min(255.0) as u8,
                c.blue().min(255.0) as u8,
            );
        }

        Self { w, h, palette }
    }

    pub fn update(&mut self, b: &mut [u32], time: f32) {

        let a = vec2(self.w as f32 / self.h as f32, 1.0);
        for y in 0..self.h {
            for x in 0..self.w {
                let fx = x as f32 / self.w as f32;
                let fy = y as f32 / self.h as f32;

                let c = vec2(fx, fy) * a * 8.0 + vec2(time * 0.3,time * 0.3);
                let k = 0.1 + (c.y + (0.148 - time).sin()).cos() + 2.4 * time;
                let w = 0.9 + (c.x + (0.628 + time).cos()).sin() - 0.7 * time;
                let d = c.length();
                let s = 7.0 * (d+w).cos() * (k+w).sin();
                let i = y * self.w + x;
                let cv = s + vec3(0.2, 0.5, 0.9);
                b[i] = col32f(cv.x,cv.y, cv.z);
                let c = self.palette[((s.cos() * 0.5 + 0.5) * 255.0) as usize % 256];
                b[i] = c;
            }
        }
    }
}
