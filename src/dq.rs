#![deny(unsafe_code)]
#![deny(missing_docs)]

//! [park](https://en.wikipedia.org/wiki/Direct-quadrature-zero_transformation)
//! and
//! [clarke](https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_transformation)
//! transformations and inverses
//!

use num::{complex::c32, Complex};

/// park inverse is nothing more then a rotation. since rotations aren't a nostd feature, we're
/// doing it the hard way
pub fn ab2dq(ab: Complex<f32>, angle: f32) -> Complex<f32> {
    let sin_angle = angle.sin();
    let cos_angle = angle.cos();
    let real = cos_angle * ab.re + sin_angle * ab.im;
    let imag = cos_angle * ab.im - sin_angle * ab.re;
    c32(real, imag)
}
