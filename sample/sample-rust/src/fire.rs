use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub enum FirePalette {
    Default,
}
#[wasm_bindgen]
pub struct StatefulFire {
    w: usize,
    h: usize,
    buffer: Vec<u32>,
    palette: Vec<u32>,
    fire: Vec<u8>,
    prev_t: f32,
}

#[wasm_bindgen]
impl StatefulFire {
    #[wasm_bindgen]
    pub fn circle(&mut self, x: u16, y: u16, r: u16) {
        if x as usize > self.w || y as usize >= self.h {
            return;
        }
        let rr = (r * r) as i32;

        for j in (y as i32 - r as i32).max(0)..(y as i32 + r as i32).min(self.h as i32 - 1) {
            for i in (x as i32 - r as i32).max(0)..(x as i32 + r as i32).min(self.w as i32 - 1) {
                let r_squared = (j - y as i32).pow(2) + (i - x as i32).pow(2);
                if r_squared < rr {
                    self.fire[j as usize * self.w + i as usize] = ((rr - r_squared.min(rr - 1)) as f32 / rr as f32 * 36. * 2.) as u8;//rng.u8(0..10);
                }
            }
        }
    }
    #[wasm_bindgen]
    pub fn get_ptr(&self) -> *const u32 { self.buffer.as_ptr() }
    #[wasm_bindgen]
    pub fn set_palette(&mut self, p:FirePalette) {
        // let mut i = 0;
        // for c in p.chunks(3) {
        //     self.palette[i].copy_from_slice(c);
        //     i += 1;
        // }
    }
    #[wasm_bindgen(constructor)]
    pub fn new(w: usize, h: usize) -> Self {
        let palette: Vec<u32> = [
            [0x07, 0x07, 0x07],
            [0x1F, 0x07, 0x07],
            [0x2F, 0x0F, 0x07],
            [0x47, 0x0F, 0x07],
            [0x57, 0x17, 0x07],
            [0x67, 0x1F, 0x07],
            [0x77, 0x1F, 0x07],
            [0x8F, 0x27, 0x07],
            [0x9F, 0x2F, 0x07],
            [0xAF, 0x3F, 0x07],
            [0xBF, 0x47, 0x07],
            [0xC7, 0x47, 0x07],
            [0xDF, 0x4F, 0x07],
            [0xDF, 0x57, 0x07],
            [0xDF, 0x57, 0x07],
            [0xD7, 0x5F, 0x07],
            [0xD7, 0x5F, 0x07],
            [0xD7, 0x67, 0x0F],
            [0xCF, 0x6F, 0x0F],
            [0xCF, 0x77, 0x0F],
            [0xCF, 0x7F, 0x0F],
            [0xCF, 0x87, 0x17],
            [0xC7, 0x87, 0x17],
            [0xC7, 0x8F, 0x17],
            [0xC7, 0x97, 0x1F],
            [0xBF, 0x9F, 0x1F],
            [0xBF, 0x9F, 0x1F],
            [0xBF, 0xA7, 0x27],
            [0xBF, 0xA7, 0x27],
            [0xBF, 0xAF, 0x2F],
            [0xB7, 0xAF, 0x2F],
            [0xB7, 0xB7, 0x2F],
            [0xB7, 0xB7, 0x37],
            [0xCF, 0xCF, 0x6F],
            [0xDF, 0xDF, 0x9F],
            [0xEF, 0xEF, 0xC7],
            [0xFF, 0xFF, 0xFF],
        ].iter().map(|&[r,g,b]|  255 << 24 | (b as u32) << 16 | (g as u32) << 8 | (r as u32))
        .collect();

        let mut fire = vec![0; w * h];

        // random bottom row
        for x in 0..w {
            let fi = (h - 1) * w + x;
            // let i = (h - 1) * w * 4 + x as usize * 4;
            //     fire[fi] = rand::random();
            //     set(b, i, &self.palette[fire[fi as usize] as usize]);
            fire[fi] = 36;
        }

        Self {
            w,
            h,
            buffer: vec![0; w*h],
            palette,
            fire,
            prev_t: 0.0,
        }
    }
    #[wasm_bindgen]
    pub fn update(&mut self, t: f32, attenuation: u8, min_x: i32, max_x: i32) {
        self.prev_t = t;
        let (w, h) = (self.w, self.h);

        let mut rng = fastrand::Rng::new();

        for x in 0..w {
            for y in 0..h {
                if y == h - 1 {
                    for x in 0..w {
                        let i = y as usize * w + x as usize;
                        self.buffer[i] = self.palette[36];
                    }
                    continue;
                }

                let fi = (y * w + x) as usize;

                let x_rand = rng.i32(min_x..=max_x);
                let fire_rand = rng.u8(0..=attenuation);

                self.fire[fi] = (self.fire
                    [(y + 1) * w + (x as i32 + x_rand as i32).clamp(0, w as i32 - 1) as usize]
                    as i32
                    - fire_rand as i32)
                    .clamp(0, 36) as u8;
                let i = y as usize * w + x as usize;

                let c = self.palette[(self.fire[fi] as usize).min(36)];
                self.buffer[i] = c;
            }
        }
    }
}
