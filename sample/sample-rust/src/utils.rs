use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::RangeInclusive;
use std::ops::Sub;

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

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

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_value(x: &JsValue);
}
pub fn as_u8_slice(v: & [u32]) -> & [u8] {
    unsafe {
        std::slice::from_raw_parts(
            v.as_ptr() as *const u8,
            v.len() * std::mem::size_of::<u32>() ,
        )
    }
}

pub fn as_u32_slice(v: &mut [u8]) -> &mut [u32] {
    unsafe {
        std::slice::from_raw_parts_mut(
            v.as_ptr() as *mut u32,
            v.len() * std::mem::size_of::<u8>() / std::mem::size_of::<u32>(),
        )
    }
}