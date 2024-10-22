#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

//! rust library for generic motor control of pmdc motors

pub mod dq;
mod estimator;
mod hall;
pub mod pid;
pub mod pt1;
