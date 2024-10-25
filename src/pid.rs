#![deny(unsafe_code)]
#![deny(missing_docs)]
#![allow(non_snake_case)]

//! rust library for generic motor control of pmdc motors

/// pid controller data struct. contains all the state of the pid
#[derive(PartialEq, Debug)]
pub struct PID {
    /// state of integrator channel
    i_chan: f32,
    /// state of differentiator channel (last updated value)
    d_chan: f32,
    /// stored most recent output of controller
    output: f32,
    /// pid configuration like amplifications
    config: PIDConfig,
}

/// configuration struct for pid contstruction
#[derive(PartialEq, Debug)]
pub struct PIDConfig {
    /// P amplification of input
    K_p: f32,
    /// Integrator amplification, a.k.a. T_n
    K_i: f32,
    /// differentiator amplification, a.k.a. T_v
    K_d: f32,
    /// high output limit
    limit_high: f32,
    /// low output limit
    limit_low: f32,
}

impl PID {
    /// create new pid controller from config
    pub fn new(cfg: PIDConfig, f_sampling_Hz: f32) -> PID {
        let newconfig = PIDConfig {
            K_i: cfg.K_i / f_sampling_Hz,
            K_d: cfg.K_d / f_sampling_Hz,
            ..cfg
        };

        PID {
            i_chan: 0f32,
            d_chan: 0f32,
            output: 0f32,
            config: newconfig,
        }
    }

    /// update controller with new controller error value
    pub fn update(&mut self, error: f32) -> f32 {
        // I channel
        if self.config.K_i != 0f32 {
            self.i_chan += self.config.K_i * error;
            // anti windup
            if self.i_chan > self.config.limit_high {
                self.i_chan = self.config.limit_high;
            } else if self.i_chan < self.config.limit_low {
                self.i_chan = self.config.limit_low;
            }
        } else {
            self.i_chan = 0f32;
        }

        // D channel
        let d = self.config.K_d * (error - self.d_chan);
        self.d_chan = error;

        // final amp
        self.output = (self.i_chan + d + error) * self.config.K_p;

        // limit output
        if self.output > self.config.limit_high {
            self.output = self.config.limit_high;
        } else if self.output < self.config.limit_low {
            self.output = self.config.limit_low;
        }

        self.output
    }

    /// reset states in pid controller to given values
    pub fn reset(&mut self, i_channel: f32, d_channel: f32) {
        self.i_chan = i_channel;
        self.d_chan = d_channel;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p() {
        let cfg = PIDConfig {
            K_p: 1f32,
            K_i: 0f32,
            K_d: 0f32,
            limit_high: 100f32,
            limit_low: -100f32,
        };

        let mut pid = PID::new(cfg, 1000f32);

        // test p channel
        assert_eq!(pid.update(1f32), 1f32);
        // check if other channels are silent
        assert_eq!(pid.update(2f32), 2f32);
        // check upper limit
        assert_eq!(pid.update(200f32), 100f32);
        // check lower limit
        assert_eq!(pid.update(-200f32), -100f32);
    }

    #[test]
    fn pi() {
        let cfg = PIDConfig {
            K_p: 1f32,
            K_i: 1f32,
            K_d: 0f32,
            limit_high: 100f32,
            limit_low: -100f32,
        };

        let mut pid = PID::new(cfg, 1f32);

        // test pi
        assert_eq!(pid.update(1f32), 2f32);
        // check integrator
        assert_eq!(pid.update(2f32), 5f32);
        // check upper limit
        assert_eq!(pid.update(200f32), 100f32);
        // check anti windup
        assert_eq!(pid.update(0f32), 100f32);
        // check lower limit
        assert_eq!(pid.update(-200f32), -100f32);
        // check anti windup
        assert_eq!(pid.update(0f32), -100f32);
        // check reset
        pid.reset(50f32, 0f32);
        assert_eq!(pid.update(0f32), 50f32);
    }

    #[test]
    fn pd() {
        let cfg = PIDConfig {
            K_p: 1f32,
            K_i: 0f32,
            K_d: 1f32,
            limit_high: 100f32,
            limit_low: -100f32,
        };

        let mut pid = PID::new(cfg, 1f32);
        // test pi
        assert_eq!(pid.update(1f32), 2f32);
        // check differentiator
        assert_eq!(pid.update(2f32), 3f32);
        // check upper limit
        assert_eq!(pid.update(200f32), 100f32);
        // check lower limit
        assert_eq!(pid.update(-200f32), -100f32);
        // check reset
        pid.reset(50f32, 50f32);
        assert_eq!(pid.update(0f32), -50f32);
    }
}
