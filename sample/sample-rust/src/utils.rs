use core::ops::Add;
use core::ops::Div;
use core::ops::Mul;
use core::ops::RangeInclusive;
use core::ops::Sub;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

/// Helper trait to implement [`lerp`] and [`remap`].
pub trait One {
    fn one() -> Self;
}
impl One for f32 {
    #[inline(always)]
    fn one() -> Self {
        1.0
    }
}
impl One for f64 {
    #[inline(always)]
    fn one() -> Self {
        1.0
    }
}
/// Helper trait to implement [`lerp`] and [`remap`].
pub trait Real:
    Copy
    + PartialEq
    + PartialOrd
    + One
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
{
}
impl Real for f32 {}
impl Real for f64 {}

pub fn lerp_byte(range: RangeInclusive<u8>, t: f32) -> u8 {
    ((1.0 - t) * *range.start() as f32 + t * *range.end() as f32).clamp(0.0, 255.0) as u8
}

/// Linear interpolation.
#[inline(always)]
pub fn lerp<R, T>(range: RangeInclusive<R>, t: T) -> R
where
    T: Real + Mul<R, Output = R>,
    R: Copy + Add<R, Output = R>,
{
    (T::one() - t) * *range.start() + t * *range.end()
}
/// Linearly remap a value from one range to another,
/// so that when `x == from.start()` returns `to.start()`
/// and when `x == from.end()` returns `to.end()`.
pub fn remap<T>(x: T, from: RangeInclusive<T>, to: RangeInclusive<T>) -> T
where
    T: Real,
{
    let t = (x - *from.start()) / (*from.end() - *from.start());
    lerp(to, t)
}

/// Like [`remap`], but also clamps the value so that the returned value is always in the `to` range.
pub fn remap_clamp<T>(x: T, from: RangeInclusive<T>, to: RangeInclusive<T>) -> T
where
    T: Real,
{
    if from.end() < from.start() {
        return remap_clamp(x, *from.end()..=*from.start(), *to.end()..=*to.start());
    }
    if x <= *from.start() {
        *to.start()
    } else if *from.end() <= x {
        *to.end()
    } else {
        let t = (x - *from.start()) / (*from.end() - *from.start());
        // Ensure no numerical inaccuracies sneak in:
        if T::one() <= t {
            *to.end()
        } else {
            lerp(to, t)
        }
    }
}

#[cfg(all(target_arch = "wasm32", debug_assertions))]
#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub(crate) fn log_value(x: &JsValue);
}

#[cfg(all(target_arch = "wasm32", not(debug_assertions)))]
pub(crate) fn log(s: &str) {}
#[cfg(all(target_arch = "wasm32", not(debug_assertions)))]
pub(crate) fn log_value(x: &JsValue) {}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn log(s: &str) {
    println!("{}", s);
}

pub(crate) fn as_u8_slice(v: &[u32]) -> &[u8] {
    unsafe {
        core::slice::from_raw_parts(
            v.as_ptr() as *const u8,
            v.len() * core::mem::size_of::<u32>(),
        )
    }
}

pub(crate) fn as_u32_slice(v: &mut [u8]) -> &mut [u32] {
    unsafe {
        core::slice::from_raw_parts_mut(
            v.as_ptr() as *mut u32,
            v.len() * core::mem::size_of::<u8>() / core::mem::size_of::<u32>(),
        )
    }
}

pub trait Lut {
    fn sin_lut(&self, lut: &SinCosLut) -> Self;
    fn cos_lut(&self, lut: &SinCosLut) -> Self;
}

impl Lut for f32 {
    fn sin_lut(&self, lut: &SinCosLut) -> f32 {
        lut.sin(*self)
    }
    fn cos_lut(&self, lut: &SinCosLut) -> f32 {
        lut.cos(*self)
    }
}

const TWO_PI: f32 = core::f32::consts::PI * 2.0;

// #[wasm_bindgen]
pub struct SinCosLut {
    sins: Vec<f32>,
    coss: Vec<f32>,
}

// #[wasm_bindgen]
impl SinCosLut {
    // #[wasm_bindgen(constructor)]
    pub fn new(sample_count: usize) -> Self {
        let mut sins = vec![0.0; sample_count];
        let mut coss = vec![0.0; sample_count];

        for i in 0..sample_count {
            let f = (i as f32 / sample_count as f32) * TWO_PI;
            sins[i] = f.sin();
            coss[i] = f.cos();
        }

        Self { sins, coss }
    }

    // #[wasm_bindgen]
    #[inline(always)]
    pub(crate) fn sin(&self, f: f32) -> f32 {
        // f.sin()
        let s = f.signum();
        let f = (f.abs() * self.sins.len() as f32 / TWO_PI) as usize;
        self.sins[f % self.sins.len()] * s
    }
    // #[wasm_bindgen]
    #[inline(always)]
    pub(crate) fn cos(&self, f: f32) -> f32 {
        // f.cos()
        let f = (f.abs() * self.coss.len() as f32 / TWO_PI) as usize;
        self.coss[f % self.coss.len()]
    }
}

pub(crate) fn colu32(c:u32) -> u32 {
    if cfg!(feature="argb") {
        // from abgr
        let (r,g,b) = (
            ((c & 0x000000FF) >> 0) as u8,
            ((c & 0x0000FF00) >> 8) as u8,
            ((c & 0x00FF0000) >> 16) as u8,
        );
        col32((r,g,b))
    } else { c }
}
pub(crate) fn col32((r, g, b): (u8, u8, u8)) -> u32 {
    if cfg!(feature="argb") {
        255 << 24 | (r as u32) << 16 | (g as u32) << 8 | (b as u32)
    } else {
        255 << 24 | (b as u32) << 16 | (g as u32) << 8 | (r as u32)
    }
}

pub(crate) fn col32f(r: u8, g: u8, b: u8, fog: u32) -> u32 {
    if cfg!(feature="argb") {
    255 << 24 | (r as u32 * fog / 100) << 16 | (g as u32 * fog / 100) << 8 | (b as u32 * fog / 100)
    } else {
    255 << 24 | (b as u32 * fog / 100) << 16 | (g as u32 * fog / 100) << 8 | (r as u32 * fog / 100)
    }
}
