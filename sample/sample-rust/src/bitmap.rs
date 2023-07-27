use bevy::prelude::IVec2;


pub(crate) struct Bitmap<'a> {
    pub data: &'a [u32],
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
    h_flip:bool,
    clip_y: i32,
) {
    let (sw, sh) = (
        (bmp.w as f32 * scale) as i32,
        (bmp.h as f32 * scale) as i32,
    );
    let inv_scale = 1.0 / scale;

    for y in (pos.y).max(0)..(pos.y + sh).min(h as i32).min(clip_y) {
        for x in (pos.x).max(0)..(pos.x + sw).min(w as i32) {
       
            let px = ((x - pos.x) as f32 * inv_scale) as usize;
            let py = ((y - pos.y) as f32 * inv_scale) as usize; let c = if h_flip{ bmp.data[bmp.w * py + bmp.w - 1 - px]}else { bmp.data[bmp.w * py + px]};
            if c != transparent {
                b[y as usize*w + x as usize] = c;
            }
        }
    }
}

// pub(crate) fn load_image(bytes: &[u8]) -> Bitmap {

// }