use core::slice;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn as_u32_slice(v: &[u8]) -> &[u32] {
    unsafe {
        core::slice::from_raw_parts(
            v.as_ptr() as *const u32,
            v.len() * core::mem::size_of::<u8>() / core::mem::size_of::<u32>(),
        )
    }
}

fn main() {
    // let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            topmost: true,
            resize: true,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let bg = image::open(
        "C:\\Users\\theor\\blog-astro\\src\\content\\blog\\rust-wasm-demo-effects\\clouds.png",
    )
    .unwrap();
let tree = image::open(
    "C:\\Users\\theor\\blog-astro\\src\\content\\blog\\rust-wasm-demo-effects\\CoconutTree.png",
)
.unwrap();
    // println!("bg {:?}", &bg.as_bytes().iter().cloned().collect::<Vec<u8>>()[0..20]);

    let mut r = sample_rust::roads2::Roads2::new(WIDTH as u32, HEIGHT as u32, as_u32_slice(bg.as_bytes()), as_u32_slice(tree.as_bytes())); //bg.as_rgb32f(), tree);
    let mut t = 0.0;
    let mut paused = true;
    let buffer = unsafe { slice::from_raw_parts(r.get_ptr(), WIDTH * HEIGHT) };
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            paused = !paused;
        }
        if t == 0.0 || !paused || window.is_key_pressed(Key::W, minifb::KeyRepeat::No) {
            r.update(
                t,
                if window.is_key_down(Key::Left) {
                    -1
                } else if window.is_key_down(Key::Right) {
                    1
                } else {
                    0
                },
                1, // if window.is_key_down(Key::Down) {
                   //     -1
                   // } else if window.is_key_down(Key::Up) {
                   //     1
                   // } else {
                   //     0
                   // },
            );
        }
        // for i in buffer.iter_mut() {
        //     *i = 0; // write something more funny here!
        // }

        t += 0.016;
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
