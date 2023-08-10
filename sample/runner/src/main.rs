use core::slice;

use image::{EncodableLayout, Rgba};
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

#[derive(Debug)]
enum Sample {
    Stars,
    Plasma,
    FireState,
    Roads,
}

enum LoopState {
    Exit,
    Switch,
}

fn update(mut window: &mut Window, mut cb: impl FnMut(&mut Window, bool, f32)) -> LoopState {
    let mut t = 0.0;
    let mut paused = true;

    loop {
        if !window.is_open() {
            return LoopState::Exit;
        }
        if window.is_key_down(Key::Escape) {
            return LoopState::Exit;
        }
        if window.is_key_pressed(Key::Tab, minifb::KeyRepeat::No) {
            return LoopState::Switch;
        }

        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            paused = !paused;
        }
        let w_pressed = window.is_key_pressed(Key::W, minifb::KeyRepeat::No);
        cb(&mut window, t == 0.0 || !paused || w_pressed, t);

        t += 0.016;
    }
    // unreachable!()
    // window.update();
    // LoopState::Continue
}
fn main() {
    // let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut sample = Sample::Stars;

    let mut window = Window::new(
        "Demo effects",
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
    window.set_position(13, 886);
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    loop {
        println!("init {sample:?}");
        let state = match sample {
            Sample::Stars => {
                let mut s = sample_rust::tunnel::Stars::new(WIDTH, HEIGHT);
                let buffer = unsafe { slice::from_raw_parts(s.get_ptr(), WIDTH * HEIGHT) };
                update(&mut window, |window, _tick, t| {
                    let (mx,my) = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap_or_default()    ;
                    s.update(t, mx / WIDTH as f32, my / HEIGHT as f32, 0.06);
                    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
                })
            }
            Sample::Plasma => {
                let mut p = sample_rust::plasma::Plasma::new(
                    WIDTH,
                    HEIGHT,
                    sample_rust::plasma::Step::All,
                    sample_rust::plasma::Palette::RainbowStepped,
                );
                let buffer = unsafe { slice::from_raw_parts(p.get_ptr(), WIDTH * HEIGHT) };
                update(&mut window, |window, _tick, t| {
                    p.update(t);
                    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
                })
            }
            Sample::FireState => {
                let mut p = sample_rust::fire::StatefulFire::new(WIDTH, HEIGHT);
                let buffer = unsafe { slice::from_raw_parts(p.get_ptr(), WIDTH * HEIGHT) };
                let mut m: (u16,u16) = (WIDTH as u16/ 2,WIDTH as u16/ 2);
                update(&mut window, |window, _tick, t| {
                    if let Some((mx,my)) = window.get_mouse_pos(minifb::MouseMode::Clamp){
// println!("{mx} {my}");
m = (mx as u16, my as u16);
                    }
                    p.update(t, 1, -1, 3);
                    p.circle(m.0,m.1, 12);
                    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
                })
            }
            Sample::Roads => {
                let bg = image::open(
                "C:\\Users\\theor\\blog-astro\\src\\content\\blog\\rust-wasm-demo-effects\\clouds.png",
            )
            .unwrap();
                let tree = image::open(
                "C:\\Users\\theor\\blog-astro\\src\\content\\blog\\rust-wasm-demo-effects\\CoconutTree.png",
            )
            .unwrap();
                // println!("bg {:?}", &bg.as_bytes().iter().cloned().collect::<Vec<u8>>()[0..20]);
                let bg = imageproc::map::map_colors(&bg, |c| Rgba([c[2], c[1], c[0], c[3]]));
                let tree = imageproc::map::map_colors(&tree, |c| Rgba([c[2], c[1], c[0], c[3]]));
                let mut r = sample_rust::roads2::Roads2::new(
                    WIDTH as u32,
                    HEIGHT as u32,
                    as_u32_slice(bg.as_bytes()),
                    as_u32_slice(tree.as_bytes()),
                ); //bg.as_rgb32f(), tree);
                let buffer = unsafe { slice::from_raw_parts(r.get_ptr(), WIDTH * HEIGHT) };

                update(&mut window, |window, tick, t| {
                    if tick {
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
                    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
                })
            }
        };
        match state {
            LoopState::Exit => {
                println!("pos {:?}", window.get_position());
                return},
            LoopState::Switch => {
                match sample {
                    Sample::Stars => sample = Sample::Plasma,
                    Sample::Plasma => sample = Sample::FireState,
                    Sample::FireState => sample = Sample::Roads,
                    Sample::Roads => sample = Sample::Stars,
                }
                window.update()
            }
        }
    }
}
