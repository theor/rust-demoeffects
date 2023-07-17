use bevy::math::{vec2, vec3};

use wasm_bindgen::prelude::wasm_bindgen;

use colorsys::{Hsl, Rgb};

use crate::utils::{lerp, SinCosLut, Lut};

#[wasm_bindgen]
pub struct Plasma {
    w: usize,
    h: usize,
    palette: Vec<u32>,
    lut: SinCosLut,
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
    pub fn new(w: usize, h: usize, lut: usize) -> Self {
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
            // palette[i] = col32(255,0,0);
        }

        Self { w, h, palette, lut: SinCosLut::new(lut), }
    }

    pub fn update(&mut self, b: &mut [u32], time: f32) {
        // Circle::new(Point::new(29, 29), 70)
        // .into_styled(PrimitiveStyle::with_stroke(Bgr888::RED, (4.0 *  (t * 5.0).sin()) as u32))
        // .draw(&mut img_display).unwrap()

        let a = vec2(self.w as f32 / self.h as f32, 1.0);
        for y in 0..self.h {
            for x in 0..self.w {
                let fx = x as f32 / self.w as f32;
                let fy = y as f32 / self.h as f32;

                let c = vec2(fx, fy) * a * 8.0 + vec2(time * 0.3,time * 0.3);
                let k = 0.1 + (c.y + (0.148 - time).sin_lut(&self.lut)).cos_lut(&self.lut) + 2.4 * time;
                let w = 0.9 + (c.x + (0.628 + time).cos_lut(&self.lut)).sin_lut(&self.lut) - 0.7 * time;
                let d = c.length();
                let s = 7.0 * (d+w).cos_lut(&self.lut) * (k+w).sin_lut(&self.lut);
                // ABGR
                // let (xt, yt) = (t * 2.0).sin_cos();
                // let cf = (127.0 + (128.0 * (fx * 32.0 + xt * 2.0).sin()))
                //     + (127.0 + (128.0 * (fy * 32.0 + (yt)).sin()));
                // let c = (cf / 2.0) as u8;
                // let shift = (t * 100.0) as usize;

                // let c = self.palette[(c as usize + shift) % 256];
                let i = y * self.w + x;
                // b[i] = c;

                
                let cv = s + vec3(0.2, 0.5, 0.9);
                // let cv = vec3(0.5 + 0.5 * cv.x.cos(), 0.5 + 0.5 * cv.y.cos(), 0.5 + 0.5 * cv.z.cos());

                // let cv = cv * (vec3(1.0, 0.7, 0.4) * cv.normalize().z.max(0.0).powi(2) + 0.75);

                b[i] = col32f(cv.x,cv.y, cv.z);
                let c = self.palette[((s.cos_lut(&self.lut) * 0.5 + 0.5) * 255.0) as usize % 256];
                b[i] = c;
            }
        }
    }
}
