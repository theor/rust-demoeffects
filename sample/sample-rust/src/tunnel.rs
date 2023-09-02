use core::f32::consts::PI;

use bevy::math::ivec2;
// use embedded_graphics::pixelcolor::raw::{RawU24, RawU32};
// use embedded_graphics::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

// use embedded_graphics::pixelcolor::Bgr888;
// use embedded_graphics::primitives::*;

// use crate::display::Display;
use crate::{utils::{remap_clamp, lerp, lerp_byte, col32, Sequence, col32f}, bitmap::{draw_bitmap, Bitmap}};
struct Star {
    angle: f32,
    dist: f32,
    p: bool,
}
#[wasm_bindgen]
pub enum StarsStep {
    Radial,
    All,
}
#[wasm_bindgen]
pub struct Stars {
    step: StarsStep,
    w: usize,
    h: usize,
    stars: Vec<Star>,
    c: (usize, usize),
    prev_v: (f32, f32),
    buffer: Vec<u32>,
    sprite: Vec<u32>,
    // prev_t: f32,
    rng: fastrand::Rng,
    // palette: Vec<u32>,
}
const COUNT: usize = 512;
#[wasm_bindgen]
impl Stars {
    #[wasm_bindgen(constructor)]
    pub fn new(w: usize, h: usize, sprite: &[u32], step: StarsStep) -> Self {
        //     let mut palette:Vec<u32> = vec![0; 256];
        //     for i in 0..palette.len() {
        //         let c = Rgb::from(Hsl::from((i as f32 / palette.len() as f32 * 360.0, 100.0, 50.0)));
        //         // let c = Rgb::from(((i as f32 / palette.len() as f32 * 255.0, 100.0, 100.0)));
        //         palette[i] = col32(c.red().min(255.0) as u8,c.green().min(255.0) as u8,c.blue().min(255.0) as u8);
        //         // palette[i] = col32(255,0,0);

        //     }
        let mut rng = fastrand::Rng::with_seed(42);
        let mut halton = Sequence::new(111);
        let mut stars = Vec::with_capacity(COUNT);
        for i in 0..COUNT {
            stars.push(Star {
                angle: rng.f32() * PI * 2.0,
                dist: rng.f32(),
                p: i % 127 == 0,
            });
        }
        Self {
            step,
            w,
            h,
            c: (w / 2, h / 2),
            stars,
            prev_v: (0.0, 0.0),
            buffer: vec![0; w*h],
            sprite: sprite.iter().cloned().collect(),
            // prev_t: 0.0,
            rng,
        }
    }
    #[wasm_bindgen]
    pub fn get_ptr(&self) -> *const u32 { self.buffer.as_ptr() }

    pub fn update(&mut self, t: f32, vx: f32, vy: f32, speed_factor: f32) {
        self.buffer.fill(0);

        let rqrt = ((self.c.0 * self.c.0 + self.c.1 * self.c.1) as f32).sqrt();
        match self.step {
            StarsStep::Radial => {
             
        let c = self.c;
           for (i, s) in self.stars.iter_mut().enumerate() {
                    s.dist += speed_factor;

                    let (sin, cos) = 
                    (s.angle.sin(), s.angle.cos());

                    let r = rqrt * s.dist;
                    let (x, y) = ((r * sin) + c.0 as f32, (r * cos) + c.1 as f32);
                    let out_x = x < 0.0 || x >= self.w as f32;
                    let out_y = y < 0.0 || y >= self.h as f32;
                    if out_x || out_y {
                        if  s.dist > 1.0 {
                            s.dist = 0.01; // * self.rng.f32();
                        }
                    } else {
                        self.buffer[x as usize + y as usize * self.w] = 0xFFFFFFFF;
                    }
                }
                return;
            },
            _ => {}
        }
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
                if s.dist > 1.0 {
                    s.dist = 0.01 * self.rng.f32();
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
                for ((bx, by),v) in line_drawing::XiaolinWu::<f32,i32>::new((xp, yp), (x, y)) {
                    if bx < 0 || by < 0 || bx >= self.w as i32 || by >= self.h as i32 { continue; }
                    let i = by as usize * self.w + bx as usize;
                    self.buffer[i]= //col32((((v * 255.) as u8), 0, 0));
                    col32f(lerp_byte(0x96..=0xff, speed), lerp_byte(0xd5..=0xff, speed), 0xFF, (v * 100.0) as u32);
                }
            }
            }
        }
    }
}

