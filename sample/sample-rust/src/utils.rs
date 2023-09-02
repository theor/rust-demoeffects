use core::ops::Add;
use core::ops::Div;
use core::ops::Mul;
use core::ops::RangeInclusive;
use core::ops::Sub;


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
#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {

    #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = console)]
    pub(crate) fn log(s: &str);
    #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = console, js_name = log)]
    pub(crate) fn log_value(x: &wasm_bindgen::JsValue);
}

#[allow(dead_code)]
#[cfg(all(target_arch = "wasm32", not(debug_assertions)))]
pub(crate) fn log(_s: &str) {}
#[cfg(all(target_arch = "wasm32", not(debug_assertions)))]
#[allow(dead_code)]
pub(crate) fn log_value(_x: &wasm_bindgen::JsValue) {}

#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn log(s: &str) {
    println!("{}", s);
}

// pub(crate) fn as_u8_slice(v: &[u32]) -> &[u8] {
//     unsafe {
//         core::slice::from_raw_parts(
//             v.as_ptr() as *const u8,
//             v.len() * core::mem::size_of::<u32>(),
//         )
//     }
// }

// pub(crate) fn as_u32_slice_mut(v: &mut [u8]) -> &mut [u32] {
//     unsafe {
//         core::slice::from_raw_parts_mut(
//             v.as_ptr() as *mut u32,
//             v.len() * core::mem::size_of::<u8>() / core::mem::size_of::<u32>(),
//         )
//     }
// }

// pub(crate) fn as_u32_slice(v: &[u8]) -> &[u32] {
//     unsafe {
//         core::slice::from_raw_parts(
//             v.as_ptr() as *const u32,
//             v.len() * core::mem::size_of::<u8>() / core::mem::size_of::<u32>(),
//         )
//     }
// }

// pub trait Lut {
//     fn sin_lut(&self, lut: &SinCosLut) -> Self;
//     fn cos_lut(&self, lut: &SinCosLut) -> Self;
// }

// impl Lut for f32 {
//     fn sin_lut(&self, lut: &SinCosLut) -> f32 {
//         lut.sin(*self)
//     }
//     fn cos_lut(&self, lut: &SinCosLut) -> f32 {
//         lut.cos(*self)
//     }
// }

// const TWO_PI: f32 = core::f32::consts::PI * 2.0;

// // #[wasm_bindgen]
// pub struct SinCosLut {
//     sins: Vec<f32>,
//     coss: Vec<f32>,
// }

// // #[wasm_bindgen]
// impl SinCosLut {
//     // #[wasm_bindgen(constructor)]
//     pub fn new(sample_count: usize) -> Self {
//         let mut sins = vec![0.0; sample_count];
//         let mut coss = vec![0.0; sample_count];

//         for i in 0..sample_count {
//             let f = (i as f32 / sample_count as f32) * TWO_PI;
//             sins[i] = f.sin();
//             coss[i] = f.cos();
//         }

//         Self { sins, coss }
//     }

//     // #[wasm_bindgen]
//     #[inline(always)]
//     pub(crate) fn sin(&self, f: f32) -> f32 {
//         // f.sin()
//         let s = f.signum();
//         let f = (f.abs() * self.sins.len() as f32 / TWO_PI) as usize;
//         self.sins[f % self.sins.len()] * s
//     }
//     // #[wasm_bindgen]
//     #[inline(always)]
//     pub(crate) fn cos(&self, f: f32) -> f32 {
//         // f.cos()
//         let f = (f.abs() * self.coss.len() as f32 / TWO_PI) as usize;
//         self.coss[f % self.coss.len()]
//     }
// }

// pub(crate) fn colu32(c:u32) -> u32 {
//     if cfg!(feature="argb") {
//         // from abgr
//         let (r,g,b) = (
//             ((c & 0x000000FF) >> 0) as u8,
//             ((c & 0x0000FF00) >> 8) as u8,
//             ((c & 0x00FF0000) >> 16) as u8,
//         );
//         col32((r,g,b))
//     } else { c }
// }
pub(crate) const fn col32((r, g, b): (u8, u8, u8)) -> u32 {
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
// copied from rust std, '*' replaced with checked_mul()
fn _checked_pow(mut base: usize, mut exp: usize) -> Option<usize> {
    let mut acc = 1usize;

    while exp > 1 {
        if (exp & 1) == 1 {
            acc = acc.checked_mul(base)?;
        }
        exp /= 2;
        base = base.checked_mul(base)?;
    }

    if exp == 1 {
        acc = acc.checked_mul(base)?;
    }

    Some(acc)
}

const D: usize = 20;
#[derive(Clone)]
pub struct Sequence {
    b: u8,
    d: [u8; D],
    r: [f32; D],
}

impl Sequence {
    /// Constructs a new `Sequence` for `base`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use halton::Sequence;
    /// let mut seq = Sequence::new(2);
    ///
    /// assert_eq!(Some(0.5), seq.next());
    /// ```
    #[inline]
    pub fn new(base: u8) -> Self {
        Sequence {
            b: base,
            d: [0; D],
            r: [0.0; D],
        }
    }

    fn pos(&self) -> Option<usize> {
        self.d
            .iter()
            .zip(1..)
            .map(|(v, i)| (*v as usize).checked_mul(i))
            .try_fold(0usize, |acc, v| acc.checked_add(v?))
    }

    fn max(&self) -> Option<usize> {
        _checked_pow(self.b as usize, D).map(|v| v - 1)
    }

    fn remaining(&self) -> Option<usize> {
        Some(self.max()? - self.pos()?)
    }
}

impl Iterator for Sequence {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut l = 0;

        self.d[l] += 1;
        if self.d[l] == self.b {
            while self.d[l] == self.b {
                self.d[l] = 0;
                l += 1;
                if l == D {
                    return None;
                }
                self.d[l] += 1;
            }
            self.r[l - 1] = (f32::from(self.d[l]) + self.r[l]) / f32::from(self.b);
            for i in (1..l).rev() {
                self.r[i - 1] = self.r[i] / f32::from(self.b);
            }
            Some(self.r[0] / f32::from(self.b))
        } else {
            Some((f32::from(self.d[0]) + self.r[0]) / f32::from(self.b))
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if let Some(remaining) = self.remaining() {
            (remaining, Some(remaining))
        } else {
            (0, None)
        }
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        if let Some(remaining) = self.remaining() {
            remaining
        } else {
            panic!("attempt to add with overflow")
        }
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        if let Some(remaining) = self.remaining() {
            self.nth(remaining - 1)
        } else {
            self.fold(None, |_, v| Some(v))
        }
    }

    #[inline]
    fn nth(&mut self, mut n: usize) -> Option<Self::Item> {
        if n > 50 {
            if let Some(mut n) = self.pos().and_then(|p| n.checked_add(p)) {
                self.d.iter_mut().for_each(|v| *v = 0);
                self.r.iter_mut().for_each(|v| *v = 0.0);
                let mut last = 0;
                while n >= usize::from(self.b) {
                    self.d[last] = n as u8 % self.b;
                    last += 1;
                    n /= usize::from(self.b);
                }
                self.d[last] = n as u8;
                for i in (1..D).rev() {
                    self.r[i - 1] = (f32::from(self.d[i]) + self.r[i]) / f32::from(self.b);
                }
                return self.next()
            }
        }
        for x in self {
            if n == 0 {
                return Some(x);
            }
            n -= 1;
        }
        None
    }
}
