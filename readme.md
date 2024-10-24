# Motor Control Library - libmotor

This crate combines motor control structs for use on embedded devices.
In the first versions, we focus on 3 phase permanent magnet DC motors (PMDC).
This type is mainly used by drone engines, robotics and also resemble grid
specs.

This crate will combine the following modules:

- [d/q transformation and inverse](https://de.wikipedia.org/wiki/D/q-Transformation)
- hall sensor to rotor position
- estimator for motor state
- [PID](https://en.wikipedia.org/wiki/Proportional%E2%80%93integral%E2%80%93derivative_controller) controller

For an implementation in an embedded system the modules are supposed to work
together in a measurement loop:

1. take electrical measures from ADC either for state estimator, or control
2. feed speed or power controller
3. take new output voltage and stuff it into inverter module

no_std is used to make this code available to bare metal embedded systems

## to do

- [x] write pid controller
  - [x] implementation
  - [x] tests
- [x] write d/q trafos
  - [x] implementation
  - [x] tests
- [x] write pt1 for filtering
  - [x] implementation
  - [x] tests
- [ ] write estimator
  - [ ] implementation
  - [ ] tests
- [ ] write hall sensor estimator
  - [x] implementation
  - [ ] tests

## Warranties and Licences

This software is licenced unter
[GPLv2](https://www.gnu.org/licenses/old-licenses/gpl-2.0.html)
open source license.
It comes without any warranties or liabilities of any form.
Use it with care.

## Contributions

If you want to contribute, please do! Rust is not my mother tongue.
Feel free to correct me.
