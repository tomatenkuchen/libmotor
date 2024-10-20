#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

//! rust library for generic motor control of pmdc motors

mod dq;
mod estimator;
mod hall;
mod pid;
