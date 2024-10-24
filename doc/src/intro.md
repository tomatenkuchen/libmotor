# libmotor - an embedded friendly 3 phase motor algorithm collection

## Motivation

Motor control on embedded systems is a two fold problem.

- setting up the hardware
  - config PWM timers for inverter stage, typically 3 phase
  - config ADC to measure motor feedback
  - precisely synchronize the two above for the measurements to work

- set up the control logic
  - feed PLL with ADC values to calculate motor speed and rotor position
  - control voltage output, either for power or speed control
  - translate voltage control to 3 phase output that the inverter can work with

The first requires you to know your chip and hardware really well.
The second wants all your control systems knowledge and 3 phase skills.

The great disaster emerges if you mix the two. Trust me.

I want to make a couple of motors spin - and I want to do it with Rust.
So please bear with me if I'm writing my code like a hobbyist and still
stick to some C++ paradigms more than I needed to. Please correct me!

This crate resembles all points from step 2. A separate implementation
on a [cheap available board](https://www.st.com/en/evaluation-tools/b-g431b-esc1.html)
will follow, and use libmotor. Don't be upset by the fact that the
proposed board is low voltage. The concepts of this board should work
for all voltages. If you happen to have a grid-fit inverter and you
are a qualified electrician and not afraid of death - try to use it.
Just don't blame me! It's on your risk. On any voltage!

## Architecture

Motor control isn't particularly hard in itself, it's just hard to keep
your controller focused on it. I only know of cases where motor control is
a hard real time application. One loop of:

1. fetch the analog data from ADC. Make it pretty
2. update rotor position and rotor speed on base of ADC input  
(or maybe hall sensor input)
3. run PID controller of speed estimation and adapt output voltage
4. give output voltage to hardware PWM

Depending on your desired motor speed, this loop should run every 50 to 20 µs,
and you're good to go. Sad part is, you need to have a controller that can
crunch this through in less than 20 to 50 µs. And every once in a while some
pesky communication routines steal your precious CPU and ruin your hard real
time.

This library gives you all the building blocks for creating this loop. You
really should benchmark your implementation on your chip (most likely with a
probe on a gpio) to see that your motor control interrupt always gets the
compute it deserves.

Most likely you have a chip that can trigger ADC conversions by timer overflows.
Otherwise you're never sure when a measurement is taken. At the end of your
ADC conversions you'll have a ADC conversion complete interrupt issued in
sync with your timer. That's a really sweet spot for your motor control routine.
Make sure it has the highest priority with the bare exception for maybe a
safety comparator firing to shut your inverter down in case of overcurrent.
If you're using normal off the shelf MOSFETs for your inverter stage, your
PWM frequency you can use is around 16 to 48 kHz if you don't want to waste all
your energy on switching losses. That translates nicely to our 20 µs goal.
If you're going with high speed GaN MOSFETs and you're driving your motor
in the high hundreds of kilohertz, you're likely to run into performance issues
with your CPU, so better separate the control loop from your PWM interrupt.
