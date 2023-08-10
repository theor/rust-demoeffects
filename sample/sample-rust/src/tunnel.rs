use core::f32::consts::PI;

use bevy::math::ivec2;
// use embedded_graphics::pixelcolor::raw::{RawU24, RawU32};
// use embedded_graphics::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

// use embedded_graphics::pixelcolor::Bgr888;
// use embedded_graphics::primitives::*;

// use crate::display::Display;
use crate::{utils::{remap_clamp, lerp, lerp_byte, col32}, bitmap::{draw_bitmap, Bitmap}};
struct Star {
    angle: f32,
    dist: f32,
    p: bool,
}
#[wasm_bindgen]
pub struct Stars {
    w: usize,
    h: usize,
    stars: Vec<Star>,
    c: (usize, usize),
    prev_v: (f32, f32),
    buffer: Vec<u32>,
    sprite: Vec<u32>,
    // prev_t: f32,
    // rng: fastrand::Rng,
    // palette: Vec<u32>,
}

const COUNT: usize = 512;
#[wasm_bindgen]
impl Stars {
    #[wasm_bindgen(constructor)]
    pub fn new(w: usize, h: usize, sprite: &[u32],) -> Self {
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
                angle: rng.f32() * PI * 4.0,
                dist: rng.f32() * 0.5,
                p: i % 127 == 0,
            });
        }
        Self {
            w,
            h,
            c: (w / 2, h / 2),
            stars,
            prev_v: (0.0, 0.0),
            buffer: vec![0; w*h],
            sprite: sprite.iter().cloned().collect(),
            // prev_t: 0.0,
            // rng,
        }
    }
    #[wasm_bindgen]
    pub fn get_ptr(&self) -> *const u32 { self.buffer.as_ptr() }

    pub fn update(&mut self, t: f32, vx: f32, vy: f32, speed_factor: f32) {
        self.buffer.fill(0);
        let speed = (t / 2.0).sin().powi(2);

        self.prev_v = (
            lerp(self.prev_v.0..=vx, 0.08),
            lerp(self.prev_v.1..=vy, 0.08),
        );

        let v = (
            (self.prev_v.0 + 0.2 * t.sin().powi(2)).clamp(0.0, 1.0),
            (self.prev_v.1 + 0.2 * t.cos().powi(2)).clamp(0.0, 1.0),
        );
        let c = (
            (self.c.0 as f32 + (self.c.0 as f32) * 1.3 * (v.0 - 0.5)) as usize, 
            (self.c.1 as f32 + (self.c.1 as f32) * 1.3 * (v.1 - 0.5)) as usize, 
        );

        let rqrt = ((self.c.0 * self.c.0 + self.c.1 * self.c.1) as f32).sqrt();
        for (i, s) in self.stars.iter_mut().enumerate() {
            let (sin, cos) = 
            // (0.1 *  i as f32, 0.23 *  i as f32);
            ((s.angle+speed).sin(), (s.angle+speed).cos());

            let rp = rqrt * s.dist;

            s.dist += speed_factor
                * s.dist.max(0.1)
                * remap_clamp(((i % 25) + 1) as f32, 0.0..=25.0, 1.0..=2.5)
                * speed;
            let r = rqrt * s.dist;

            let (xp, yp) = ((rp * sin) + c.0 as f32, (rp * cos) + c.1 as f32);
            let (x, y) = ((r * sin) + c.0 as f32, (r * cos) + c.1 as f32);
            let out_x = x < 0.0 || x >= self.w as f32;
            let out_y = y < 0.0 || y >= self.h as f32;
            if out_x || out_y {
                if out_x && out_y {
                    s.dist = 0.01; // * self.rng.f32();
                }
            } else {
                if s.p {
                    draw_bitmap(
                        &mut self.buffer,
                        self.w,
                        self.h,
                        ivec2(x as i32,y as i32)- ivec2((100./2.*s.dist) as i32, (100./2.*s.dist) as i32),&Bitmap { data: self.sprite.as_slice(), w: 100, h: 100 },
                        0,
                        s.dist, false, std::i32::MAX
                     
                    );
                } else {
                for (bx, by) in line_drawing::Bresenham::new((xp as i32, yp as i32), (x as i32, y as i32)) {
                    let i = by as usize * self.w + bx as usize;
                    self.buffer[i]= col32((lerp_byte(0x96..=0xff, speed), lerp_byte(0xd5..=0xff, speed), 0xFF));
                }
            }
            }
        }
    }
}

