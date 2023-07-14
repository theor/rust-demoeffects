use line_drawing::Bresenham;
use wasm_bindgen::prelude::wasm_bindgen;

use bevy::math::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


struct Seg {
    index: usize,
    p1: Vec3,
    p2: Vec3,
}

#[wasm_bindgen]
pub struct Roads {
    size: IVec2,
    segments: Vec<Seg>,
    position: f32,
}

fn col32(r: u8, g: u8, b: u8) -> u32 {
    255 << 24 | (b as u32) << 16 | (g as u32) << 8 | (r as u32)
}

const SEGMENT_LENGTH: i32 = 200; // length of a single segment
const RUMBLE_LENGTH: i32 = 3; // number of lanes
const FIELD_OF_VIEW: f32 = 100.0; // angle (degrees) for field of view
const CAMERA_HEIGHT: f32 = 2000.0; // z height of camera
const ROAD_WIDTH: f32 = 2000.0; // z height of camera
const DRAW_DISTANCE: i32 = 300;

#[wasm_bindgen]
impl Roads {
    #[wasm_bindgen(constructor)]
    pub fn new(w: usize, h: usize) -> Self {
        let mut segments = Vec::with_capacity(500);
        for n in 0..segments.capacity() {
            segments.push(Seg {
                index: n,
                p1: Vec3::Z * n as f32 * SEGMENT_LENGTH as f32,
                p2: Vec3::Z * (n + 1) as f32 * SEGMENT_LENGTH as f32,
            });
        }

        log(&format!("segs: {}", segments.len()));

        Self {
            size: ivec2(w as i32, h as i32),
            segments,
            position: 0.0,
        }
    }

    fn project(&self, p: Vec3, cam_pos: Vec3, camera_depth: f32, road_width: f32) -> Option<IVec3> {
        let cam = p - cam_pos;

        let hw = self.size.x as f32 / 2.0;
        let hh = self.size.y as f32 / 2.0;
        if cam.z < camera_depth {
            return None;
        }
        let screen_scale = camera_depth / cam.z;
        let screen = IVec3::new(
            (hw + screen_scale * cam.x * hw) as i32,
            (hh - screen_scale * cam.y * hh) as i32,
            (screen_scale * road_width * hw) as i32,
        );
        Some(screen)
    }

    fn find_segment(&mut self, z: f32) -> usize {
        let l = self.segments.len();
        (z / SEGMENT_LENGTH as f32).floor() as usize % l
    }

    pub fn update(&mut self, b: &mut [u32], time: f32) {
        let camera_depth: f32 = 1.0 / ((FIELD_OF_VIEW / 2.0) * std::f32::consts::PI / 180.0).tan();
        let resolution: f32 = self.size.y as f32 / 480.0;

        let player_z: f32 = CAMERA_HEIGHT * camera_depth;

        let track_length = 500 * SEGMENT_LENGTH;

        let base_segment = self.find_segment(self.position);

        b.fill(0xff00ff00);
        let mut maxy = self.size.y as i32;

        for n in 0..DRAW_DISTANCE {
            let seg = (self.segments[base_segment].index + n as usize) % self.segments.len();
            log(&format!("n {n} seg {seg} pos {}", self.position));

            let dark = (n / RUMBLE_LENGTH) % 2 == 0;

            let cam = Vec3::new(
                0.0 /* playerX */ * ROAD_WIDTH,
                CAMERA_HEIGHT,
                900.0, // t*10.0, /* position - (segment.looped ? trackLength : 0) */
            );

            let (s1, s2) = {
                let seg = &self.segments[seg];
                (
                    self.project(seg.p1, cam, camera_depth, ROAD_WIDTH),
                    self.project(seg.p2, cam, camera_depth, ROAD_WIDTH),
                )
            };

            // behind us
            if let (Some(s1), Some(s2)) = (s1, s2) {
                if s2.y > maxy {
                    // clip by (already rendered) segment
                    continue;
                }
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

                const RUMBLE_WIDTH: f32 = 0.05;
                for (l, r) in Bresenham::new((tl.x, tl.y), (bl.x, bl.y))
                    .zip(Bresenham::new((tr.x, tr.y), (br.x, br.y)))
                {
                    if l.1 < 0 || l.1 > (self.size.y as i32) - 1 {
                        break;
                    }
                    let lw = r.0 - l.0;
                    for p in Bresenham::new(l, r) {
                        if !(0..self.size.x as i32 - 1).contains(&p.0) {
                            continue;
                        }
                        let horizontal_ratio = (p.0 - l.0) as f32 / lw as f32;
                        b[(p.1 * self.size.x + p.0) as usize] = if horizontal_ratio < RUMBLE_WIDTH
                            || horizontal_ratio > 1.0 - RUMBLE_WIDTH
                        {
                            if dark {
                                col32(255, 0, 0)
                            } else {
                                0xffffffff
                            }
                        } else if dark && (0.47..0.53).contains(&horizontal_ratio) {
                            0xffffffff
                        } else {
                            if dark {
                                0xff555555
                            } else {
                                0xffaaaaaa
                            }
                        };
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
        ()
    }
}

//  1
// 2 3
fn fill_bottom_flat_triangle(b: &mut [u32], v1: IVec2, v2: IVec2, v3: IVec2, w: usize, c: u32) {
    let invslope1: i32 = (v2.x - v1.x) / (v2.y - v1.y);
    let invslope2: i32 = (v3.x - v1.x) / (v3.y - v1.y);

    let mut curx1: i32 = v1.x;
    let mut curx2: i32 = v1.x;

    for scanline_y in v1.y..=v2.y {
        for p in line_drawing::Bresenham::new((curx1, scanline_y), (curx2, scanline_y)) {
            b[p.1 as usize * w + p.0 as usize] = c;
        }
        curx1 += invslope1;
        curx2 += invslope2;
    }
}

// 1 2
//  3
fn fill_top_flat_triangle(b: &mut [u32], v1: IVec2, v2: IVec2, v3: IVec2, w: usize, c: u32) {
    let invslope1 = (v3.x - v1.x) / (v3.y - v1.y);
    let invslope2 = (v3.x - v2.x) / (v3.y - v2.y);

    let mut curx1 = v3.x;
    let mut curx2 = v3.x;

    for scanline_y in (v1.y..v3.y).rev() {
        for p in line_drawing::Bresenham::new((curx1, scanline_y), (curx2, scanline_y)) {
            b[p.1 as usize * w + p.0 as usize] = c;
        }
        curx1 -= invslope1;
        curx2 -= invslope2;
    }
}
