use core::slice;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    // let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let mut r = sample_rust::roads2::Roads2::new(WIDTH as u32, HEIGHT as u32);
    let mut t = 0.0;
    let mut paused = true;
    let buffer = unsafe { slice::from_raw_parts(r.get_ptr(), WIDTH * HEIGHT) };
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No)
           { paused = !paused;}
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
                if window.is_key_down(Key::Down) {
                    -1
                } else if window.is_key_down(Key::Up) {
                    1
                } else {
                    0
                },
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
