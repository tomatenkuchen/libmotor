#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

//! rust library for generic motor control of pmdc motors

pub mod dq;
pub mod hall;
pub mod motor;
pub mod pid;
pub mod pll;
pub mod pt1;
