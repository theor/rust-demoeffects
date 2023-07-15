use wasm_bindgen::{prelude::wasm_bindgen, Clamped};
use wasm_bindgen::JsCast;

use bevy::math::*;
use web_sys::ImageData;

use crate::utils::{log, as_u32_slice, log_value, as_u8_slice};

struct Seg {
    index: usize,
    p1: Vec3,
    // p2: Vec3,
}

#[wasm_bindgen]
pub struct Roads2 {
    buffer: Vec<u32>,
    size: IVec2,
    segments: Vec<Seg>,
    position: f32,
}

// fog is 0..100
fn col32f(r: u8, g: u8, b: u8, fog: u32) -> u32 {
    255 << 24 | (b as u32 * fog / 100) << 16 | (g as u32 * fog / 100) << 8 | (r as u32 * fog / 100)
}

const SEGMENT_LENGTH: i32 = 200; // length of a single segment
const RUMBLE_LENGTH: i32 = 3; // number of lanes
const FIELD_OF_VIEW: f32 = 100.0; // angle (degrees) for field of view
const CAMERA_HEIGHT: f32 = 1000.0; // z height of camera
const ROAD_WIDTH: f32 = 2000.0; // z height of camera
const DRAW_DISTANCE: i32 = 300;
const SEGMENT_COUNT: i32 = 500;
#[wasm_bindgen]
impl Roads2 {
    #[wasm_bindgen(constructor)]
    pub fn new( w:u32, h: u32) -> Self {
  

        // let document = web_sys::window().unwrap().document().unwrap();
        // let canvas = document.get_element_by_id(canvas_id).unwrap();
        // let canvas: web_sys::HtmlCanvasElement = canvas
        //     .dyn_into::<web_sys::HtmlCanvasElement>()
        //     .map_err(|_| ())
        //     .unwrap();
        // let ctx = canvas
        //     .get_context("2d")
        //     .unwrap()
        //     .unwrap()
        //     .dyn_into::<web_sys::CanvasRenderingContext2d>()
        //     .unwrap();

        let mut segments = Vec::with_capacity(SEGMENT_COUNT as usize);
        for n in 0..segments.capacity() {
            segments.push(Seg {
                index: n,
                p1: vec3(
                    800.0 * (n as f32 / 10.0).sin() + (n as f32 / 121.0).cos() * 100.0,
                    0.0,
                    n as f32 * SEGMENT_LENGTH as f32,
                ),
                // p2: vec3(
                //     400.0 * ((n+1) as f32 / 10.0).sin(),
                //     0.0,
                //     (n + 1) as f32 * SEGMENT_LENGTH as f32,
                // ),
            });
        }

        log(&format!("segs: {}", segments.len()));

        Self {
            size: ivec2(w as i32, h as i32),
            segments,
            position: 0.0,
            buffer: vec![0; (w*h) as usize],
        }
    }

    fn project(
        &self,
        p: Vec3,
        cam_pos: Vec3,
        camera_depth: f32,
        road_width: f32,
        bottom: bool,
    ) -> Option<IVec3> {
        let cam = p - cam_pos;

        let hw = self.size.x as f32 / 2.0;
        let hh = self.size.y as f32 / 2.0;
        if cam.z < camera_depth {
            return None;
        }
        let screen_scale = camera_depth / cam.z;
        let y = hh - screen_scale * cam.y * hh;
        let screen = IVec3::new(
            (hw + screen_scale * cam.x * hw) as i32,
            if !bottom { y.floor() } else { y.ceil() } as i32,
            (screen_scale * road_width * hw) as i32,
        );
        // log(&format!("{p:#} {y} {screen_scale}"));
        Some(screen)
    }

    fn find_segment(&mut self, z: f32) -> usize {
        let l = self.segments.len();
        (z as usize / SEGMENT_LENGTH as usize) as usize % l
    }

    pub fn update(&mut self, ctx: &web_sys::CanvasRenderingContext2d, time: f32, dir_x: i8, dir_y: i8) {
        // let mut bb = self.buffer.data().0;
        // bb.fill(0xff);
        // let b = as_u32_slice(&mut bb);
        let track_length = SEGMENT_COUNT * SEGMENT_LENGTH;

        self.position += 150.0 * dir_y as f32; // * ((time * 4.0).sin().powi(2) + 0.5);
        while self.position > track_length as f32 {
            self.position -= track_length as f32;
        }

        let camera_depth: f32 = 1.0 / ((FIELD_OF_VIEW / 2.0) * std::f32::consts::PI / 180.0).tan();
        let resolution: f32 = self.size.y as f32 / 480.0;

        let player_z: f32 = CAMERA_HEIGHT * camera_depth;

        let mut x = 0.0;
        // let mut dx = 0.0;

        let base_segment = self.find_segment(self.position);
        // log(&format!("pos {} base {base_segment}", self.position));

        // sky
        self.buffer[0..=(self.size.y >> 1) as usize * self.size.x as usize].fill(0xff5dc3ff);
        // b.fill(0xffffffff);
        // self.buffer.data().fill(0xff);
        // log_value(&ctx);
        // grass
        self.buffer[(self.size.y >> 1) as usize * self.size.x as usize..].fill(
            // 0xff0000ff
            0xff7a9c86,
        );
        let mut maxy = self.size.y as i32;

        for n in 0..DRAW_DISTANCE {
            // if n > 10 { break }
            // if n <= 8 { continue }
            // if n != 10 { continue }
            let seg_i = (self.segments[base_segment].index + n as usize) % self.segments.len();

            // if n == 0 {
            //     log(&format!("t {time} n {n} seg {seg} pos {}", self.position));
            // }

            let dark = (seg_i as i32 / RUMBLE_LENGTH) % 2 == 0;
            // let dark2 = (seg  as i32 / RUMBLE_LENGTH) % 4 == 0;

            let (s1, s2) = {
                let seg = &self.segments[seg_i];
                let seg2 = &self.segments[(seg_i + 1) % self.segments.len()];
                let looped = seg.index < self.segments[base_segment].index;
                // if looped { log("looped")}
                let cam = Vec3::new(
                    0.0 /* playerX */ * ROAD_WIDTH - x,
                    CAMERA_HEIGHT,
                    self.position - if looped { track_length as f32 } else { 0.0 },
                );
                let cam2 = cam - vec3(0.0, 0.0, 0.0);
                (
                    self.project(seg.p1, cam, camera_depth, ROAD_WIDTH, false),
                    self.project(seg2.p1, cam2, camera_depth, ROAD_WIDTH, false),
                )
            };

            // x += dx;

            // behind us
            if let (Some(s1), Some(s2)) = (s1, s2) {
                // if s2.y > maxy {
                //     // clip by (already rendered) segment
                //     continue;
                // }

                let fog_d = n as f32 / DRAW_DISTANCE as f32;
                let fog = (100.0 / (fog_d * fog_d * 5.0).exp()) as u32;
                // log(&fog.to_string());

                maxy = s2.y;

                let tl = ivec2(
                    s2.x - s2.z, //.clamp(0, self.w as i32 - 1),
                    s2.y,        //.clamp(0, self.h as i32 - 1),
                );
                let bl = ivec2(
                    s1.x - s1.z, //.clamp(0, self.w as i32 - 1),
                    s1.y,        //.clamp(0, self.h as i32 - 1),
                );
                let tr = ivec2(
                    s2.x + s2.z, //.clamp(0, self.w as i32 - 1),
                    s2.y,        //.clamp(0, self.h as i32 - 1),
                );
                let br = ivec2(
                    s1.x + s1.z, //.clamp(0, self.w as i32 - 1),
                    s1.y,        //.clamp(0, self.h as i32 - 1),
                );

                let dy = s1.y - s2.y;
                let l_slope = ((bl - tl).x as f32) / dy as f32;
                let r_slope = ((br - tr).x as f32) / dy as f32;

                // log(&format!("{dy} {tl} {tr} {bl} {br}"));

                const RUMBLE_WIDTH: f32 = 0.05;
                // fix that. different slopes mean non horizontal lines means incomplete rect
                // for (l, r) in Bresenham::new((tl.x, tl.y), (bl.x, bl.y))
                //     .zip(Bresenham::new((tr.x, tr.y), (br.x, br.y)))
                for y in 0..dy {
                    let py = s2.y + y;
                    if py >= self.size.y {
                        break;
                    }
                    let l = bl + ivec2((y as f32 * l_slope) as i32, y);
                    let r = br + ivec2((y as f32 * r_slope) as i32, y);
                    // log(&format!("  {l:?} {r:?}"));
                    // log(&format!("    py {py}"));
                    let lw = r.x - l.x;
                    for p in l.x..r.x {
                        if !(0..self.size.x as i32).contains(&p) {
                            continue;
                        }
                        let horizontal_ratio = (p - l.x) as f32 / lw as f32;

                        let c = if horizontal_ratio < RUMBLE_WIDTH
                            || horizontal_ratio > 1.0 - RUMBLE_WIDTH
                        {
                            if !dark {
                                col32f(163, 14, 2, fog) // red rumble
                            } else {
                                col32f(0xff, 0xff, 0xff, fog)
                            }
                        } else if dark && (0.49..0.51).contains(&horizontal_ratio) {
                            // center line
                            col32f(0xff, 0xff, 0xff, fog)
                        } else {
                            // road segment
                            if dark {
                                col32f(0x92, 0x97, 0x93, fog)
                            } else {
                                col32f(0x9c, 0x9e, 0x9b, fog)
                            }
                        };

                        self.buffer[((py) * self.size.x + p) as usize] = c; // (c as f32 * fog) as u32;
                    }
                }

                // fill_bottom_flat_triangle(b, tl, bl, br, self.w, 0xff0000ff);
                // fill_top_flat_triangle(b, tl, tr, br, self.w, 0xffff00ff);

                // for x in (s1.x - s1.z).max(0)..(s2.x + s2.z).min(self.w as i32 - 1) {
                //     b[(s1.y).clamp(0, self.h as i32 - 1) as usize * self.w + x as usize] =
                //         0xffffffff;
                // }
            }
        }
        ctx.put_image_data(&ImageData::new_with_u8_clamped_array_and_sh( Clamped(&as_u8_slice(&self.buffer)), self.size.x as u32, self.size.y as u32).unwrap(), 0.0, 0.0).unwrap();

        // ctx.put_image_data(&self.buffer, 0.0, 0.0).unwrap();
    }
}

// //  1
// // 2 3
// fn fill_bottom_flat_triangle(b: &mut [u32], v1: IVec2, v2: IVec2, v3: IVec2, w: usize, c: u32) {
//     let invslope1: i32 = (v2.x - v1.x) / (v2.y - v1.y);
//     let invslope2: i32 = (v3.x - v1.x) / (v3.y - v1.y);

//     let mut curx1: i32 = v1.x;
//     let mut curx2: i32 = v1.x;

//     for scanline_y in v1.y..=v2.y {
//         for p in line_drawing::Bresenham::new((curx1, scanline_y), (curx2, scanline_y)) {
//             b[p.1 as usize * w + p.0 as usize] = c;
//         }
//         curx1 += invslope1;
//         curx2 += invslope2;
//     }
// }

// // 1 2
// //  3
// fn fill_top_flat_triangle(b: &mut [u32], v1: IVec2, v2: IVec2, v3: IVec2, w: usize, c: u32) {
//     let invslope1 = (v3.x - v1.x) / (v3.y - v1.y);
//     let invslope2 = (v3.x - v2.x) / (v3.y - v2.y);

//     let mut curx1 = v3.x;
//     let mut curx2 = v3.x;

//     for scanline_y in (v1.y..v3.y).rev() {
//         for p in line_drawing::Bresenham::new((curx1, scanline_y), (curx2, scanline_y)) {
//             b[p.1 as usize * w + p.0 as usize] = c;
//         }
//         curx1 -= invslope1;
//         curx2 -= invslope2;
//     }
// }
