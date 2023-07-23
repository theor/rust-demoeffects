use bevy::math::vec2;

use bracket_noise::prelude::{FastNoise, NoiseType, FractalType};
use wasm_bindgen::prelude::wasm_bindgen;

use colorsys::{Hsl, Rgb};

use crate::utils::{col32, lerp, log, remap};

#[wasm_bindgen]
pub enum Step {
    FixedCircle,
    ShiftedCircle,
    Perturbation,
    All,
}
#[wasm_bindgen]
pub enum Palette {
    Greyscale,
    GreyscaleLooped,
    Colors,
    ColorsStepped,
    Rainbow,
    RainbowStepped,
}

const PALETTE_SIZE: usize = 256;

#[wasm_bindgen]
pub struct Plasma {
    w: usize,
    h: usize,
    palette: Vec<u32>,
    step: Step,
    buffer: Vec<u32>,
}

#[wasm_bindgen]
impl Plasma {
    #[wasm_bindgen(constructor)]
    pub fn new(w: usize, h: usize, step: Step, pal: Palette) -> Self {
        let palette: Vec<u32> = vec![0; PALETTE_SIZE];

        let mut s = Self {
            w,
            h,
            palette,
            step,
            buffer: vec![0; (w*h) as usize],
        };

        s.set_palette(pal);

        s
    }

    pub fn set_palette(&mut self, pal: Palette) {
        

        for i in 0..PALETTE_SIZE {
            let f = i as f32 / PALETTE_SIZE as f32;

            let c = match pal {
                Palette::Colors => {
                    let ff = if i < PALETTE_SIZE / 2 {
                        f * 2.0
                    } else {
                        1.0 - ((f - 0.5) * 2.0)
                    };
                    col32(Rgb::from(Hsl::from((
                        lerp(203.0..=31.0, ff / 2.0),
                        82.0,
                        ((ff / 2.0 + 0.5).powi(2) * 100.0).clamp(10.0, 100.0),
                    ))).into())
                }
                Palette::Greyscale => col32((i as u8, i as u8, i as u8)),
                Palette::GreyscaleLooped => {
                    let c = if i < PALETTE_SIZE / 2 {
                        i as i32 * 2
                    } else {
                        PALETTE_SIZE as i32 - ((i - PALETTE_SIZE / 2) as i32 * 2)
                    };
                    col32(Rgb::from((c, c, c)).into())
                }
                Palette::ColorsStepped => {
                    let ff = if i < PALETTE_SIZE / 2 {
                        f * 2.0
                    } else {
                        1.0 - ((f - 0.5) * 2.0)
                    };
                    let stepped = (ff * 4.0).round() / (4.0);
                    col32(Rgb::from(Hsl::from((
                        lerp(203.0..=31.0, stepped / 2.0),
                        82.0,
                        ((stepped / 2.0 + 0.5).powi(2) * 100.0).clamp(10.0, 100.0),
                    ))).into())
                    // let ff = ((f * 256.0) as i32 / 16 * 16  % 256) as i32;
                    // Rgb::from((ff,ff,ff))
                }
                Palette::Rainbow => {
                    col32(Rgb::from(Hsl::from((
                        360.0 * f,
                        82.0,
                        80.0,
                    ))).into())
                }
                Palette::RainbowStepped => {
                    col32(Rgb::from(Hsl::from((
                        360.0 * (f * 8.0).round() / (8.0),
                        82.0,
                        80.0,
                    ))).into())
                }
            };

            self.palette[i] = c;
        }

    }

    pub fn get_ptr(&self) -> *const u32 {
        self.buffer.as_ptr()
    }

    // Abgr to rgba
    pub fn get_palette(&self) -> Vec<u32> {
        self.palette
            .iter()
            .map(|c| {
                255 | // a
             (*c & 0xFF0000)  >> 8| // b
              (*c & 0xFF00)<< 8  | // g
               (*c & 0xFF) << 24 // r
            })
            .collect::<Vec<u32>>()
    }

    pub fn update(&mut self, time: f32) {
        log(&format!("asdasd {time}"));

        let mut noise = FastNoise::seeded(42);
        noise.set_noise_type(NoiseType::SimplexFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(1);
        noise.set_fractal_gain(0.6);
        noise.set_fractal_lacunarity(0.25);
        noise.set_frequency(1.1);
        let a = vec2(self.w as f32 / self.h as f32, 1.0);
        for y in 0..self.h {
            for x in 0..self.w {
                let i = y * self.w + x;
                let fx = x as f32 / (self.w - 1) as f32;
                let fy = y as f32 / (self.h - 1) as f32;
                self.buffer[i] = self.palette[match self.step {
                    Step::FixedCircle => {
                        remap(
                            (vec2(fx - 0.5, fy - 0.5) * 16.0).length().sin(),
                            -1.0..=1.0,
                            0.0..=255.0,
                        ) as usize
                            % 256
                    }
                    Step::ShiftedCircle => {
                        remap(
                            (vec2(fx - 0.5, fy - 0.5) * 16.0).length().sin() + time ,
                           -1.0..=1.0,
                            0.0..=255.0,
                        ) as usize
                            % 256
                    }
                    // ((((fx * 10.0).cos() * (fy * 10.0).sin()) * 0.25 + 0.5) * 255.0) as usize % 256,
                    Step::Perturbation => {
                        remap(
                            (vec2(fx - 0.25 - time.sin() * 0.2, fy - 0.3 - time.cos() * 0.3) * 16.0).length().sin() + time ,
                           -1.0..=1.0,
                            0.0..=255.0,
                        ) as usize
                            % 256
                    }
                    Step::All => {
                        let w = noise.get_noise(fx * 3.0 + (time / 3.0).sin(), fy + (time / 7.0).cos().powi(2));
                        remap(
                            noise.get_noise(fx + w + time.cos(),fy + w.powi(3) + (time/1.7).sin())
                                + time * (fx * fy + 0.1) / 6.0,
                            0.0..=0.7,
                            0.0..=255.0,
                        ) as usize
                            % 256
                        // let c = vec2(fx, fy) * a * 8.0 + vec2(time * 0.3, time * 0.3);
                        // let k = 0.1 + (c.y + (0.148 - time).sin()).cos() + 2.4 * time;
                        // let w = 0.9 + (c.x + (0.628 + time).cos()).sin() - 0.7 * time;
                        // let d = c.length();
                        // let s = 7.0 * (d + w).cos() * (k + w).sin();
                        // ((s.cos() * 0.5 + 0.5) * 255.0) as usize % 256
                    }
                }];
            }
        }
    }
}
