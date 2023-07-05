use std::{f32::consts::PI, vec};

use colorsys::{Hsl, Rgb};
use wasm_bindgen::prelude::wasm_bindgen;

struct Star {
    angle: f32,
    dist: f32,
}
#[wasm_bindgen]
pub struct Stars {
    w: usize,
    h: usize,
    stars: Vec<Star>,
    c: (usize, usize),
    prev_t: f32,
    rng: fastrand::Rng,
    // palette: Vec<u32>,
}

fn col32(r: u8, g: u8, b: u8) -> u32 {
    255 << 24 | (b as u32) << 16 | (g as u32) << 8 | (r as u32)
}
const COUNT:usize = 128;
#[wasm_bindgen]
impl Stars {
    #[wasm_bindgen(constructor)]
    pub fn new(w: usize, h: usize) -> Self {
        //     let mut palette:Vec<u32> = vec![0; 256];
        //     for i in 0..palette.len() {
        //         let c = Rgb::from(Hsl::from((i as f32 / palette.len() as f32 * 360.0, 100.0, 50.0)));
        //         // let c = Rgb::from(((i as f32 / palette.len() as f32 * 255.0, 100.0, 100.0)));
        //         palette[i] = col32(c.red().min(255.0) as u8,c.green().min(255.0) as u8,c.blue().min(255.0) as u8);
        //         // palette[i] = col32(255,0,0);

        //     }
        let mut rng = fastrand::Rng::new();
        let mut stars = Vec::with_capacity(COUNT);
        for i in 0..COUNT {
            stars.push(Star {
                angle: rng.f32() * PI * 2.0,
                dist: rng.f32(),
            });
        }
        Self {
            w,
            h,
            c: (w / 2, h / 2),
            stars,
            prev_t: 0.0,
            rng,
        }
    }

    pub fn update(&mut self, b: &mut [u32], t: f32) {
        
        b.fill(0xFF000000);
        for s in self.stars.iter_mut() {
            s.dist += 0.1 * s.dist.max(0.01);
            let r = ((self.c.0 * self.c.0 + self.c.1 * self.c.1) as f32).sqrt() * s.dist;
            let (x, y) = (
                (r * s.angle.sin()) + self.c.0 as f32,
                (r * s.angle.cos()) + self.c.1 as f32,
            );
            if x < 0.0 || x >= self.w as f32 || y < 0.0 || y > self.h as f32 {
                s.dist = 0.0;
            } else {
                b[y as usize * self.w + x as usize] = 0xFFFFFFFF;
            }
        }
        // for y in 0..self.h {
        //     for x in 0..self.w {
        //         let i = y * self.w + x;
        //         let fx = x as f32 / self.w as f32;
        //         let fy = y as f32 / self.h as f32;
        //         // ABGR
        //         let (xt,yt) = (t*2.0).sin_cos();
        //         let cf = (127.0 + (128.0 * (fx * 32.0 + xt * 2.0).sin()))
        //             + (127.0 + (128.0 * (fy * 32.0 + (yt )).sin()));
        //         let c = (cf /2.0) as u8;
        //         let shift = (t * 100.0) as usize;
        //         b[i] = self.palette[(c as usize + shift) % 256];
        //         // b[i+1] = 127;
        //         // b[i+2] = 127;
        //         // b[i+3] = 255;
        //     }
        // }
    }
}
