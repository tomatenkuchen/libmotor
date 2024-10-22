#![deny(unsafe_code)]
#![deny(missing_docs)]

//! rust library for generic motor control of pmdc motors

/// PT1 state
pub struct PT1 {
    /// integrator state
    i_chan: f32,
    /// most recent output
    output: f32,
    /// configuration
    config: PT1Config,
}

/// configuration structure for PT1
pub struct PT1Config {
    /// integration amplifyer
    K_p: f32,
    /// time constant
    T: f32,
}

impl PT1 {
    /// create new PT1
    pub fn new(cfg: PT1Config, f_sample_Hz: f32) -> PT1 {
        let newconfig = PT1Config {
            T: cfg.T * f_sample_Hz,
            ..cfg
        };

        PT1 {
            i_chan: 0f32,
            output: 0f32,
            config: newconfig,
        }
    }

    /// update filter with new data
    pub fn update(&mut self, input: f32) -> f32 {
        self.i_chan -= self.i_chan / self.config.T;
        self.i_chan += input * self.config.K_p;
        self.output = self.i_chan / self.config.T;

        self.output
    }

    /// reset filter to given value. if in doublt, use 0
    pub fn reset(&mut self, reset_value: f32) {
        self.i_chan = reset_value * self.config.T;
        self.output = reset_value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1() {
        let cfg = PT1Config { K_p: 1f32, T: 1f32 };

        let mut pt1 = PT1::new(cfg, 1000f32);

        // test initial value
        assert_eq!(pt1.output, 0f32);
        // test p channel for 1 second, which is 1000 iterations at 1kHz sampling rate
        for _ in 0..1000 {
            pt1.update(1f32);
        }
        assert_eq!(pt1.output, 0.62f32);
    }
}
