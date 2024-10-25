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

use crate::motor::Mechanical;

const HALL_SUM_TO_SECTOR_NO: [i8; 6] = [0, 2, 1, 4, 5, 3];

/// hall sensor rotor state estimation struct
#[derive(PartialEq, Debug)]
pub struct Hall {
    recent_sector: i8,
    speed_recent: f32,
}

impl Hall {
    /// run this method on a state change of any hall sensor, at best in a fast interrupt
    /// put in the logical state of the hall sensors and the time between now and the last
    /// interrupt to be able to calculate speed
    pub fn interrupt_service_routine(
        &mut self,
        hall_1: bool,
        hall_2: bool,
        hall_3: bool,
        t_hall_state: f32,
    ) -> Mechanical {
        // calc hall number
        let hall_no = (hall_1 as u8 + (hall_2 as u8) * 2 + (hall_3 as u8) * 4) as usize - 1;
        // find new sector by table
        let sector = HALL_SUM_TO_SECTOR_NO[hall_no];
        // find out direction by compare previous sector
        let sector_diff = sector - self.recent_sector;
        // calc speed
        let speed_radpers = core::f32::consts::PI / 3f32 / t_hall_state;
        // calc acceleration
        let acceleration_radperss = (speed_radpers - self.speed_recent) / t_hall_state;

        let motorstate: Mechanical = match sector_diff {
            // clockwise operation
            1 => Mechanical {
                angle: sector as f32 * core::f32::consts::PI / 3f32 - core::f32::consts::PI / 6f32,
                speed: speed_radpers,
                acceleration: acceleration_radperss,
            },
            // counterclockwise operation
            -1 => Mechanical {
                angle: sector as f32 * core::f32::consts::PI / 3f32 + core::f32::consts::PI / 6f32,
                speed: -speed_radpers,
                acceleration: acceleration_radperss,
            },
            // overflow from sector 5 to 0
            -5 => Mechanical {
                angle: core::f32::consts::PI * 22f32 / 12f32,
                speed: speed_radpers,
                acceleration: acceleration_radperss,
            },
            // overflow from sector 0 to 5
            5 => Mechanical {
                angle: core::f32::consts::PI * 22f32 / 12f32,
                speed: -speed_radpers,
                acceleration: acceleration_radperss,
            },

            // error case, this should not happen
            _ => Mechanical {
                angle: 0f32,
                speed: 0f32,
                acceleration: 0f32,
            },
        };

        // TODO: change result type to result type to make sure there's a response to sensor error
        // (like all false or all true inputs) and sector jumps (like 0 to 2)

        self.speed_recent = motorstate.speed;
        self.recent_sector = sector;

        motorstate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// test for clockwise motion
    #[test]
    fn hall_interrupt_routine_cw() {
        // init hall sensor routine with initial sector
        let mut hall = Hall {
            recent_sector: 0i8,
            speed_recent: 0f32,
        };

        // simulate new sector in clockwise direction (sector 1)
        let res_cw_1ms = hall.interrupt_service_routine(true, true, false, 1e-3f32);
        let sol_cw_1ms = Mechanical {
            angle: core::f32::consts::PI / 6f32,
            speed: core::f32::consts::PI / 3f32 / 1e-3f32,
            acceleration: core::f32::consts::PI / 3f32 / 1e-6f32,
        };
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_1ms.angle,
            sol_cw_1ms.angle,
            epsilon = 0.001
        ));
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_1ms.speed,
            sol_cw_1ms.speed,
            epsilon = 0.001
        ));
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_1ms.acceleration,
            sol_cw_1ms.acceleration,
            epsilon = 0.2
        ));

        // simulate new sector in clockwise direction (sector 2)
        let res_cw_2ms = hall.interrupt_service_routine(false, true, false, 2e-3f32);
        let sol_cw_2ms = Mechanical {
            angle: core::f32::consts::PI * 3f32 / 6f32,
            speed: core::f32::consts::PI / 3f32 / 2e-3f32,
            acceleration: -core::f32::consts::PI / 3f32 / 4e-6f32,
        };
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_2ms.angle,
            sol_cw_2ms.angle,
            epsilon = 0.001
        ));
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_2ms.speed,
            sol_cw_2ms.speed,
            epsilon = 0.001
        ));
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_2ms.acceleration,
            sol_cw_2ms.acceleration,
            epsilon = 0.1
        ));

        // simulate new sector in clockwise direction (sector 3)
        let res_cw_2ms = hall.interrupt_service_routine(false, true, true, 2e-3f32);
        let sol_cw_2ms = Mechanical {
            angle: core::f32::consts::PI * 5f32 / 6f32,
            speed: core::f32::consts::PI / 3f32 / 2e-3f32,
            acceleration: 0f32,
        };
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_2ms.angle,
            sol_cw_2ms.angle,
            epsilon = 0.001
        ));
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_2ms.speed,
            sol_cw_2ms.speed,
            epsilon = 0.001
        ));
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_2ms.acceleration,
            sol_cw_2ms.acceleration,
            epsilon = 0.1
        ));

        // simulate new sector in clockwise direction (sector 4)
        let res_cw_2ms = hall.interrupt_service_routine(false, false, true, 2e-3f32);
        let sol_cw_2ms = Mechanical {
            angle: core::f32::consts::PI * 7f32 / 6f32,
            speed: core::f32::consts::PI / 3f32 / 2e-3f32,
            acceleration: 0f32,
        };
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_2ms.angle,
            sol_cw_2ms.angle,
            epsilon = 0.001
        ));
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_2ms.speed,
            sol_cw_2ms.speed,
            epsilon = 0.001
        ));
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_2ms.acceleration,
            sol_cw_2ms.acceleration,
            epsilon = 0.1
        ));

        // simulate new sector in clockwise direction (sector 5)
        let res_cw_2ms = hall.interrupt_service_routine(true, false, true, 2e-3f32);
        let sol_cw_2ms = Mechanical {
            angle: core::f32::consts::PI * 9f32 / 6f32,
            speed: core::f32::consts::PI / 3f32 / 2e-3f32,
            acceleration: 0f32,
        };
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_2ms.angle,
            sol_cw_2ms.angle,
            epsilon = 0.001
        ));
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_2ms.speed,
            sol_cw_2ms.speed,
            epsilon = 0.001
        ));
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_2ms.acceleration,
            sol_cw_2ms.acceleration,
            epsilon = 0.1
        ));

        // simulate new sector in clockwise direction (sector 0)
        let res_cw_2ms = hall.interrupt_service_routine(true, false, false, 2e-3f32);
        let sol_cw_2ms = Mechanical {
            angle: core::f32::consts::PI * 11f32 / 6f32,
            speed: core::f32::consts::PI / 3f32 / 2e-3f32,
            acceleration: 0f32,
        };
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_2ms.angle,
            sol_cw_2ms.angle,
            epsilon = 0.001
        ));
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_2ms.speed,
            sol_cw_2ms.speed,
            epsilon = 0.001
        ));
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_2ms.acceleration,
            sol_cw_2ms.acceleration,
            epsilon = 0.1
        ));
    }

    /// counterclockwise motion. I leave out tests for speed and acceleration since it was tested
    /// already in cw
    #[test]
    fn hall_interrupt_routine_ccw() {
        // init hall sensor routine with initial sector
        let mut hall = Hall {
            recent_sector: 0i8,
            speed_recent: 0f32,
        };

        // simulate new sector in clockwise direction (sector 5)
        let res_cw_1ms = hall.interrupt_service_routine(true, false, true, 1e-3f32);
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_1ms.angle,
            core::f32::consts::PI * 11f32 / 6f32,
            epsilon = 0.001
        ));
        // simulate new sector in clockwise direction (sector 4)
        let res_cw_1ms = hall.interrupt_service_routine(false, false, true, 1e-3f32);
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_1ms.angle,
            core::f32::consts::PI * 9f32 / 6f32,
            epsilon = 0.001
        ));
        // simulate new sector in clockwise direction (sector 3)
        let res_cw_1ms = hall.interrupt_service_routine(false, true, true, 1e-3f32);
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_1ms.angle,
            core::f32::consts::PI * 7f32 / 6f32,
            epsilon = 0.001
        ));
        // simulate new sector in clockwise direction (sector 2)
        let res_cw_1ms = hall.interrupt_service_routine(false, true, false, 1e-3f32);
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_1ms.angle,
            core::f32::consts::PI * 5f32 / 6f32,
            epsilon = 0.001
        ));
        // simulate new sector in clockwise direction (sector 1)
        let res_cw_1ms = hall.interrupt_service_routine(true, true, false, 1e-3f32);
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_1ms.angle,
            core::f32::consts::PI * 3f32 / 6f32,
            epsilon = 0.001
        ));
        // simulate new sector in clockwise direction (sector 0)
        let res_cw_1ms = hall.interrupt_service_routine(true, false, false, 1e-3f32);
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_1ms.angle,
            core::f32::consts::PI * 1f32 / 6f32,
            epsilon = 0.001
        ));
        // simulate new sector in clockwise direction (sector 5)
        let res_cw_1ms = hall.interrupt_service_routine(true, false, true, 1e-3f32);
        assert!(float_cmp::approx_eq!(
            f32,
            res_cw_1ms.angle,
            core::f32::consts::PI * 11f32 / 6f32,
            epsilon = 0.001
        ));
    }
}
