use bevy::prelude::IVec2;


pub(crate) struct Bitmap {
    pub data: &'static [u32],
    pub w: usize,
    pub h: usize,
}
pub(crate) fn draw_bitmap(
    b: &mut Vec<u32>,
    w: usize,
    h: usize,
    pos: IVec2,
    bmp: &Bitmap,
    transparent: u32,
    scale: f32,
) {
    let (sw, sh) = (
        (bmp.w as f32 * scale).floor() as i32,
        (bmp.h as f32 * scale).floor() as i32,
    );
    let is = 1.0 / scale;

    for y in (pos.y).max(0)..(pos.y + sh).min(h as i32) {
        for x in (pos.x).max(0)..(pos.x + sw).min(w as i32) {
            let px = ((x - pos.x) as f32 * is).floor() as usize;
            let py = ((y - pos.y) as f32 * is).floor() as usize;
            let c = bmp.data[bmp.w * py + px];
            if c != transparent {
                b[y as usize*w + x as usize] = c;
            }
        }
    }

    // let scale_ceil = scale.ceil();
    // let mut skipped = 0;
    // let mut total = 0;

    // for y in 0..bmp.h {

    //     for x in 0..bmp.w {

    //         let c = bmp.data[bmp.w * y + x];
    //         if c != transparent {
    //             for sy in 0..scale_ceil as usize {
    //                 let py = (((scale * y as f32)).ceil() as i32 + pos.y as i32 + sy as i32) as i32;
    //                 total += scale_ceil as usize;
    //                 if py < 0 || py as usize >= h { skipped += scale_ceil as usize; continue }
    //                 for sx in 0..scale_ceil as usize {
    //                     let px = (x as f32 * scale).ceil() as i32 + sx as i32 + pos.x as i32;
    //                     if px < 0 || px as usize >= w { skipped += 1; continue }
    //                     b[
    //                         w *  py as usize +
    //                         px as usize
    //                     ] = c;
    //                 }
    //             }
    //         }
    //     }
    // }
    // println!("skipped {skipped}/{total}")
}