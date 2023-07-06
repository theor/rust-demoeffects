use std::f32::consts::PI;

// use embedded_graphics::pixelcolor::raw::{RawU24, RawU32};
// use embedded_graphics::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

// use embedded_graphics::pixelcolor::Bgr888;
// use embedded_graphics::primitives::*;

// use crate::display::Display;
use crate::utils::{remap_clamp, lerp, lerp_byte};
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
    prev_v: (f32, f32),
    // prev_t: f32,
    // rng: fastrand::Rng,
    // palette: Vec<u32>,
}

const COUNT: usize = 512;
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
        for _ in 0..COUNT {
            stars.push(Star {
                angle: rng.f32() * PI * 4.0,
                dist: rng.f32() * 0.5,
            });
        }
        Self {
            w,
            h,
            c: (w / 2, h / 2),
            stars,
            prev_v: (0.0, 0.0),
            // prev_t: 0.0,
            // rng,
        }
    }

    pub fn update(&mut self, b: &mut [u8], t: f32, vx: f32, vy: f32, speed_factor: f32) {
        b.fill(0);
        // let mut img_display: Display<'_, Bgr888> =
            // Display::new(Size::from((self.w as u32, self.h as u32)), b);
        // img_display.clear(Bgr888::BLACK).unwrap();
        // Circle::new(Point::new(29, 29), 70)
        // .into_styled(PrimitiveStyle::with_stroke(Bgr888::RED, (4.0 *  (t * 5.0).sin()) as u32))
        // .draw(&mut img_display).unwrap();

        let speed = (t / 2.0).sin().powi(2);
        // let col = Bgr888::new(
        //     (remap_clamp(speed, 0.0..=1.0, 150.0..=255.0)) as u8,
        //     (remap_clamp(speed, 0.0..=1.0, 213.0..=255.0)) as u8,
        //     227,
        // );

        // let vx = (t*1.5).cos();
        // let vy = (t*2.2).sin();

self.prev_v = (
    lerp(self.prev_v.0..=vx, 0.05),
    lerp(self.prev_v.1..=vy, 0.05),
);
        let c = (
            (self.c.0 as f32 + (self.c.0 as f32) * 1.3 * (self.prev_v.0 - 0.5)) as usize, 
            (self.c.1 as f32 + (self.c.1 as f32) * 1.3 * (self.prev_v.1 - 0.5)) as usize, 
        );

        let rqrt = ((self.c.0 * self.c.0 + self.c.1 * self.c.1) as f32).sqrt();
        for (i, s) in self.stars.iter_mut().enumerate() {
            let (sin, cos) = 
            // (0.1 *  i as f32, 0.23 *  i as f32);
            (s.angle.sin(), s.angle.cos());

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


                // Line::new(Point::new(xp as i32, yp as i32), Point::new(x as i32, y as i32))
                // .draw_styled(&PrimitiveStyle::with_stroke(Bgr888::from(RawU24::from(col)), remap_clamp(s.dist, 0.0..=1.0, 1.0..=3.0) as u32), &mut img_display).unwrap();
                
                for (bx, by) in line_drawing::Bresenham::new((xp as i32, yp as i32), (x as i32, y as i32)) {
                    let i = (by as usize * self.w + bx as usize) * 4;
                    b[i..i + 4].copy_from_slice(&[ lerp_byte(0x96..=0xff, speed), lerp_byte(0xd5..=0xff, speed), 0xFF, 0xFF,]);

                }

                // let i = idx(xp, yp, self.w);
                // b[i..i + 4].copy_from_slice(&[ 0x96, 0xd5, 0xFF, 0xFF,]);
                // let i = idx(x, y, self.w);
                // b[i..i + 4].copy_from_slice(&[0xFF, 0xFF, 0x77, 0xFF]);

                // b[i..i + 4].copy_from_slice(&[ 0x00, 0x00, 0xbb, 0xFF,]); // r g _ r
                // b[y as usize * self.w + x as usize] = 0xFFFFFFFF;
            }
        }
    }
}

// 6.72
// 3.9 no trig

// 2.15 no blit

fn idx(x: f32, y: f32, w: usize) -> usize {
    (y as usize * w + x as usize) * 4
}
