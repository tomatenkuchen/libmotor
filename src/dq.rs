#![deny(unsafe_code)]
#![deny(missing_docs)]

//! [park](https://en.wikipedia.org/wiki/Direct-quadrature-zero_transformation)
//! and
//! [clarke](https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_transformation)
//! transformations and inverses
//!

use micromath::F32Ext;
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

/// park transformation on the other hand is a rotation in the opposite direction, so let's reuse
pub fn dq2ab(dq: Complex<f32>, angle: f32) -> Complex<f32> {
    ab2dq(dq, -angle)
}

/// clarke transformation means transforming 2d voltages and currents into 3 phase voltage/current
/// information. backwards is the inverse transformation.
pub fn ab2abc(ab: Complex<f32>) -> [f32; 3] {
    let mut abc = [0f32; 3];
    let sqrt3: f32 = 3f32.sqrt();
    abc[0] = 1.5f32 * ab.re;
    abc[1] = 0.75f32 * (ab.im * sqrt3 - ab.re);
    abc[2] = -0.75f32 * (ab.im * sqrt3 + ab.re);

    abc
}

/// clarke transformation from 3 phase to 2d
pub fn abc2ab(abc: [f32; 3]) -> Complex<f32> {
    let invsqrt3 = 1f32 / 3f32.sqrt();
    let re = abc[0];
    let im = invsqrt3 * (abc[0] + 2f32 * abc[1]);

    c32(re, im)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ab2dq() {
        let ab = c32(325f32, 0f32);
        let dq = ab2dq(ab, core::f32::consts::PI / 2f32);
        assert!(float_cmp::approx_eq!(f32, dq.re, 0f32, epsilon = 0.1));
        assert!(float_cmp::approx_eq!(f32, dq.im, -325f32, epsilon = 0.1));
    }

    #[test]
    fn test_dq2ab() {
        let dq = c32(325f32, 0f32);
        let ab = dq2ab(dq, core::f32::consts::PI / 2f32);
        assert!(float_cmp::approx_eq!(f32, ab.re, 0f32, epsilon = 0.1));
        assert!(float_cmp::approx_eq!(f32, ab.im, 325f32, epsilon = 0.1));
    }

    #[test]
    fn test_abc2ab() {
        let abc = [325f32, -162.5f32, -162.5f32];
        let ab = abc2ab(abc);
        assert!(float_cmp::approx_eq!(f32, ab.re, 325f32, epsilon = 0.1));
        assert!(float_cmp::approx_eq!(f32, ab.im, 0f32, epsilon = 0.1));
    }

    #[test]
    fn test_ab2abc() {
        let ab = c32(216.666f32, 0f32);
        let abc = ab2abc(ab);
        assert!(float_cmp::approx_eq!(f32, abc[0], 325f32, epsilon = 0.1));
        assert!(float_cmp::approx_eq!(f32, abc[1], -162.5f32, epsilon = 0.1));
        assert!(float_cmp::approx_eq!(f32, abc[2], -162.5f32, epsilon = 0.1));
    }
}
