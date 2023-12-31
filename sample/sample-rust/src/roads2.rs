use wasm_bindgen::prelude::wasm_bindgen;

use bevy::math::*;
use crate::{utils::{col32, col32f, lerp}, bitmap::{draw_bitmap, Bitmap}};

struct Seg {
    index: usize,
    p1: Vec3,
    screen: (IVec3, f32),
    clip: i32,
    // p2: Vec3,
}

#[wasm_bindgen]
pub struct Roads2 {
    background: Vec<u32>,
    tree: Vec<u32>,
    buffer: Vec<u32>,
    size: IVec2,
    segments: Vec<Seg>,
    position: f32,
    dir: Vec2,
    camera_depth: f32,
    player_y: f32,
}


// fog is 0..100

const COCONUT: IVec2 = IVec2 { 
    x: 30,
    y: 80,
};

const SEGMENT_LENGTH: i32 = 200; // length of a single segment
const RUMBLE_LENGTH: i32 = 3; // number of lanes
const FIELD_OF_VIEW: f32 = 100.0; // angle (degrees) for field of view
const CAMERA_HEIGHT: f32 = 1000.0; // z height of camera
const ROAD_WIDTH: f32 = 2000.0; // z height of camera
const DRAW_DISTANCE: i32 = 300;
const SEGMENT_COUNT: i32 = 501;
#[wasm_bindgen]
impl Roads2 {
    #[wasm_bindgen(constructor)]
    pub fn new(w: u32, h: u32, bg: &[u32], tree: &[u32]) -> Self {
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
                clip: 0,
                index: n,
                p1: vec3(
                    800.0 * (n as f32 / 10.0).sin() + (n as f32 / 121.0).cos() * 600.0,
                    (n as f32 / 31.0).sin() * (n as f32 / 42.0).cos() * SEGMENT_LENGTH as f32 * 7.0,
                    n as f32 * SEGMENT_LENGTH as f32,
                ),
                screen: Default::default(),
                // p2: vec3(
                //     400.0 * ((n+1) as f32 / 10.0).sin(),
                //     0.0,
                //     (n + 1) as f32 * SEGMENT_LENGTH as f32,
                // ),
            });
        }

        let bg = bg.iter().cloned()
        // .map(|u| {
        //     0xFF000000 |  // A
        //     // arGb -> abGr
        //     (u & 0xFF00) >> 0 |
        //     // argB ->aBgr
        //     ((u & 0xFF) << 16) |
        //     // aRgb-> abgR
        //     ((u & 0xFF0000)>> 16 )
        // })
        .collect();
        // crate::utils::log(&format!("segs: {:?}", bg));

        Self {
            player_y: 0.0,
            size: ivec2(w as i32, h as i32),
            segments,
            position: 0.0,
            buffer: vec![0; (w * h) as usize],
            dir: Vec2::ZERO,
            camera_depth: 1.0 / ((FIELD_OF_VIEW / 2.0) * core::f32::consts::PI / 180.0).tan(),
            background: bg,
            tree: tree.iter().cloned().collect(),
        }
    }

    pub fn get_ptr(&self) -> *const u32 {
        self.buffer.as_ptr()
    }

    fn project(
        &self,
        p: Vec3,
        cam_pos: Vec3,
        camera_depth: f32,
        road_width: f32,
        bottom: bool,
    ) -> Option<(IVec3, f32)> {
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
        Some((screen, screen_scale))
    }

    fn find_segment(&mut self, z: f32) -> usize {
        let l = self.segments.len();
        (z as usize / SEGMENT_LENGTH as usize) as usize % l
    }

    pub fn update(&mut self, _time: f32, dir_x: i8, dir_y: i8) {
        let track_length = SEGMENT_COUNT * SEGMENT_LENGTH;
        self.dir.x = (self.dir.x + dir_x as f32 * 0.1).clamp(-1.0, 1.0);

        self.position += 150.0 * dir_y as f32; // * ((time * 4.0).sin().powi(2) + 0.5);
        while self.position > track_length as f32 {
            self.position -= track_length as f32;
        }
              while self.position < 0 as f32 {
            self.position += track_length as f32;
        }

        // let resolution: f32 = self.size.y as f32 / 480.0;

        let player_z: f32 = CAMERA_HEIGHT * self.camera_depth;
        let player_percent = (self.position+player_z) % SEGMENT_LENGTH as f32 / SEGMENT_LENGTH as f32;
        let player_segment = self.find_segment(self.position + player_z);
        self.player_y = lerp( self.segments[player_segment].p1.y..=self.segments[(player_segment+1) % SEGMENT_COUNT as usize].p1.y, player_percent);

        let x = 0.0;
        // let mut dx = 0.0;

        let base_segment = self.find_segment(self.position);
        // crate::utils::log(&format!(
        //     "pos {} base {base_segment} {}",
        //     self.position, self.segments[base_segment].index
        // ));

        self.buffer.fill(col32((0, 147, 255)));
        // sky
        draw_bitmap(&mut self.buffer,
            self.size.x as usize,
            self.size.y as usize, ivec2(0, -20), 
            &Bitmap { data: self.background.as_slice(), w: 505, h: 200 },
            // &sprites::CLOUDS,
             0x00000000, 1.5, false, std::i32::MAX);
      
        // self.buffer.fill(col32((0, 147, 255)));
        // grass
        // self.buffer[(self.size.y >> 1) as usize * self.size.x as usize..].fill(
        //     // 0xff0000ff
        //     colu32(if dark { 0xff7a6c86} else {0xffcedeef}),
        // );
        let mut maxy = self.size.y as i32;

        for n in 0..DRAW_DISTANCE {
            // if n > 10 { break }
            // if n <= 8 { continue }
            // if n != 10 { continue }
            let seg_i = (self.segments[base_segment].index + n as usize) % self.segments.len();
{
    self.segments[seg_i].clip = maxy;

}
            // if n == 0 {
            //     log(&format!("t {time} n {n} seg {seg} pos {}", self.position));
            // }

            let dark = (seg_i as i32 / RUMBLE_LENGTH) % 2 == 0;
            // let dark2 = (seg  as i32 / RUMBLE_LENGTH) % 4 == 0;

            let (s1, s2) = {
                let seg = &self.segments[seg_i];
                let seg2 = &self.segments[(seg_i + 1) % self.segments.len()];
                let looped = seg.index < self.segments[base_segment].index;
                let cam = Vec3::new(
                    self.dir.x * ROAD_WIDTH - x,
                    CAMERA_HEIGHT + self.player_y,
                    self.position - if looped { track_length as f32 } else { 0.0 },
                );
                let cam2 = cam - vec3(0.0, 0.0, 0.0);
                (
                    self.project(seg.p1, cam, self.camera_depth, ROAD_WIDTH, false),
                    self.project(seg2.p1, cam2, self.camera_depth, ROAD_WIDTH, false),
                )
            };
            // x += dx;

            // behind us
            if let (Some((s1, z1)), Some((s2, _z2))) = (s1, s2) {
                self.segments[seg_i].screen = (s1, z1);
                if s2.y > maxy || s2.y > s1.y {
                    // clip by (already rendered) segment
                    continue;
                }

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
                for y in 0..dy {
                    let py = s2.y + y;
                    if py >= self.size.y {
                        break;
                    }
                    let l = bl + ivec2((y as f32 * l_slope) as i32, y);
                    let r = br + ivec2((y as f32 * r_slope) as i32, y);
                    let lw = r.x - l.x;
                    for p in 0..(l.x).min(self.size.x) {
                        self.buffer[((py) * self.size.x + p) as usize] = if dark {
                            col32((239, 222, 206))
                        } else {
                            col32((230, 214, 197))
                        };
                    }
                    for p in r.x..self.size.x as i32 {
                        self.buffer[((py) * self.size.x + p) as usize] = if dark {
                            col32((239, 222, 206))
                        } else {
                            col32((230, 214, 197))
                        };
                    }
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
            }
        }

        for n in (0..DRAW_DISTANCE).rev() {
            let seg_i = (self.segments[base_segment].index + n as usize) % self.segments.len();
            let seg = &self.segments[seg_i];
            let is_tree = seg_i % 5 == 0 && n < 80;
            if is_tree {
                let spr_w =
                    (seg.screen.1 * seg.screen.0.z as f32 * self.size.x as f32 * 0.1).max(0.1);
                // println!("{n} {} {spr_w}", seg.screen.0 );
                let coconut_bitmap = 
                Bitmap { w: COCONUT.x as usize, h: COCONUT.y as usize, data:self.tree.as_slice() };
                draw_bitmap(
                    &mut self.buffer,
                    self.size.x as usize,
                    self.size.y as usize,   
                    seg.screen.0.xy()
                        - ivec2(
                            seg.screen.0.z + ((COCONUT.x + 50) as f32 * spr_w / 2.0) as i32,
                            ((COCONUT.y as f32 - 5.0) * spr_w) as i32,
                        ),
                        &coconut_bitmap,
                    0,
                    // 3
                    spr_w,
                    false,
                    seg.clip,
                );
                draw_bitmap(
                    &mut self.buffer,
                    self.size.x as usize,
                    self.size.y as usize,
                    seg.screen.0.xy()
                        - ivec2(
                            - seg.screen.0.z - ((COCONUT.x + 10) as f32 * spr_w / 2.0) as i32,
                            ((COCONUT.y as f32 - 5.0) * spr_w) as i32,
                        ),&coconut_bitmap,
                    0,
                    // 3
                    spr_w,
                    true,
                    seg.clip,
                );
            }
        }
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
