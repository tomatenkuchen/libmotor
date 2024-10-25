#![deny(unsafe_code)]
#![deny(missing_docs)]

//! PMDC motor definition for state and config

use num::Complex;

/// motor state
pub struct Config {
    /// stator phase resistance R1
    pub resistance: f32,
    /// motor inductance. complex for motors with asymmetrical rotor inductances. If your rotor
    /// happens to be a pure non-iron magnet, you likely have no complex inductance, or better,
    /// it's the same for imaginary and real. Put in your measured inductance in both fields. The
    /// equations you work with should make it happen.
    pub inductance: num::Complex<f32>,
    /// magnetic flux: the "strength" of your magnet if you will. Careful: it depends on your
    /// stator as well, so make sure you measure it build in, or calculate it. Best case, your
    /// manufacturer tells you.
    pub flux: f32,
    /// rotor inertia
    pub inertia: f32,
}

/// electrical state of machine
pub struct Electrical {
    /// voltage on machine in dq
    pub voltage: num::Complex<f32>,
    /// current on machine in dq
    pub current: num::Complex<f32>,
}
impl Electrical {
    /// calculates active power from electrical state
    pub fn calc_power_active(&self) -> f32 {
        self.voltage.im * self.current.im
    }

    /// calculates reactive power from electrical state
    pub fn calc_power_reactive(&self) -> f32 {
        self.voltage.re * self.current.re
    }

    /// calculates apparent power from electrical state
    pub fn calc_power_apparent(&self) -> Complex<f32> {
        self.voltage * self.current
    }
}

/// mechanical state of motor
pub struct Mechanical {
    /// rotor angle in rad
    pub angle: f32,
    /// rotor speed in rad per second
    pub speed: f32,
    /// rotor acceleration in rad per second²
    pub acceleration: f32,
}

impl Mechanical {
    /// integrate all states by time. Make sure there's correct data in the struct fields.
    pub fn calc_state_iteration(&mut self, t_delta: f32) {
        self.speed += self.acceleration * t_delta;
        self.angle += self.speed * t_delta;
        // do a modulo for angle to keep it within 0 and 2 pi
        if self.angle >= 2f32 * core::f32::consts::PI {
            self.angle -= 2f32 * core::f32::consts::PI;
        } else if self.angle <= 0f32 {
            self.angle -= core::f32::consts::PI;
        }
    }
}

/// combined motor state
pub struct Motor {
    /// electrical data
    pub elec: Electrical,
    /// mechanical data
    pub mech: Mechanical,
}
