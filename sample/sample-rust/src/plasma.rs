use embedded_graphics::pixelcolor::raw::RawU24;
use embedded_graphics::{prelude::*, pixelcolor::Rgb888};

use embedded_graphics::pixelcolor::Bgr888;
use embedded_graphics::primitives::*;
use std::vec;
use wasm_bindgen::prelude::wasm_bindgen;

use colorsys::{Hsl, Rgb};

use crate::display::Display;

#[wasm_bindgen]
pub struct Plasma {
    w: usize,
    h: usize,
    palette: Vec<u32>,
}

fn col32(r: u8, g: u8, b: u8) -> u32 {
    255 << 24 | (b as u32) << 16 | (g as u32) << 8 | (r as u32)
}

#[wasm_bindgen]
impl Plasma {
    #[wasm_bindgen(constructor)]
    pub fn new(w: usize, h: usize) -> Self {
        let mut palette: Vec<u32> = vec![0; 256];
        for i in 0..palette.len() {
            let c = Rgb::from(Hsl::from((
                i as f32 / palette.len() as f32 * 360.0,
                100.0,
                50.0,
            )));
            // let c = Rgb::from(((i as f32 / palette.len() as f32 * 255.0, 100.0, 100.0)));
            palette[i] = col32(
                c.red().min(255.0) as u8,
                c.green().min(255.0) as u8,
                c.blue().min(255.0) as u8,
            );
            // palette[i] = col32(255,0,0);
        }

        Self { w, h, palette }
    }

    pub fn update(&mut self, b: &mut [u8], t: f32) {

        let mut img_display: Display<'_, Bgr888> = Display::new(Size::from((self.w as u32, self.h as u32)), b);
        // img_display.clear(Bgr888::BLUE).unwrap();
        // Circle::new(Point::new(29, 29), 70)
        // .into_styled(PrimitiveStyle::with_stroke(Bgr888::RED, (4.0 *  (t * 5.0).sin()) as u32))
        // .draw(&mut img_display).unwrap()
        for y in 0..self.h {
            for x in 0..self.w {
                let i = y * self.w + x;
                let fx = x as f32 / self.w as f32;
                let fy = y as f32 / self.h as f32;
                // ABGR
                let (xt, yt) = (t * 2.0).sin_cos();
                let cf = (127.0 + (128.0 * (fx * 32.0 + xt * 2.0).sin()))
                    + (127.0 + (128.0 * (fy * 32.0 + (yt)).sin()));
                let c = (cf / 2.0) as u8;
                let shift = (t * 100.0) as usize;

                let c = self.palette[(c as usize + shift) % 256];
                let rgb: Bgr888 = RawU24::from(c).into();
                Pixel(Point::new(x as i32, y as i32), rgb).draw(&mut img_display);
                // let rgb_slice = &[rgb.r(), rgb.g(), rgb.b(), 255];
                // b[i * 4..(i + 1) * 4].copy_from_slice(rgb_slice);
            }
        }
    }
}
