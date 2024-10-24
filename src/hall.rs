#![deny(unsafe_code)]
#![deny(missing_docs)]
#![allow(non_snake_case)]

//! hall sensor rotor estimator
//!
//! hall sensors on motors enable fast rotor state analysis without much
//! compute. On a 3 phase system, we usually have 3 hall sensors mounted
//! close to the rotor or a rotor mounted magnet in a 120° degree arrangement.
//!
//! We can create a truth table to identiry the rotor position.
//!
//! | sector number | rotor angle | hall sensor 1 | hall sensor 2 | hall sensor 3 | hall sum |
//! | ------------- | ----------- | ------------- | ------------- | ------------- | -------- |
//! | 0             | 330° -  30° | true          | false         | false         | 1        |
//! | 1             |  30° -  90° | true          | true          | false         | 3        |
//! | 2             |  90° - 150° | false         | true          | false         | 2        |
//! | 3             | 150° - 210° | false         | true          | true          | 6        |
//! | 4             | 210° - 270° | false         | false         | true          | 4        |
//! | 5             | 270° - 330° | true          | false         | true          | 5        |
//!
//! If we've got the hall states, we have a rough measure for the rotor position.
//!
//!
//!

use crate::motor::Mechanical;

const HALL_SUM_TO_SECTOR_NO: [u8; 6] = [0, 2, 1, 4, 5, 3];

/// hall sensor rotor state estimation struct
pub struct Hall {
    recent_sector: u8,
}

impl Hall {
    /// run this method on a state change of any hall sensor, at best in a fast interrupt
    /// put in the logical state of the hall sensors and the time between now and the last
    /// interrupt to be able to calculate speed
    pub fn hall_state_interrupt(
        &mut self,
        hall_1: bool,
        hall_2: bool,
        hall_3: bool,
        t_hall_state: f32,
    ) -> Mechanical {
        // calc hall number
        let hall_no = (hall_1 as u8 + (hall_2 as u8) * 2 + (hall_3 as u8) * 4) as usize;
        // find new sector by table
        let sector = HALL_SUM_TO_SECTOR_NO[hall_no];
        // find out direction by compare previous sector
        let sector_diff = sector as i8 - self.recent_sector as i8;
        // shadow copy sector to be f32 to make compiler happy
        let sector = sector as f32;

        let motorstate: Mechanical = match sector_diff {
            // clockwise operation
            1 => Mechanical {
                angle: sector * core::f32::consts::PI / 6f32 - core::f32::consts::PI / 12f32,
                speed: core::f32::consts::PI / 6f32 / t_hall_state,
                acceleration: 0f32,
            },
            // counterclockwise operation
            -1 => Mechanical {
                angle: sector * core::f32::consts::PI / 6f32 + core::f32::consts::PI / 12f32,
                speed: core::f32::consts::PI / -6f32 / t_hall_state,
                acceleration: 0f32,
            },
            // overflow from sector 5 to 0
            -5 => Mechanical {
                angle: core::f32::consts::PI * 11f32 / 12f32,
                speed: core::f32::consts::PI / -6f32 / t_hall_state,
                acceleration: 0f32,
            },
            // overflow from sector 0 to 5
            5 => Mechanical {
                angle: core::f32::consts::PI * 12f32,
                speed: core::f32::consts::PI / -6f32 / t_hall_state,
                acceleration: 0f32,
            },

            // error case, this should not happen
            _ => Mechanical {
                angle: 0f32,
                speed: 0f32,
                acceleration: 0f32,
            },
        };

        motorstate
    }
}
