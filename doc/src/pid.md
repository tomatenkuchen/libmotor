# PID controller

## Intro

Here again we have a fundamental building block for control systems. And - as
always - [Wikipedia](https://en.wikipedia.org/wiki/Proportional%E2%80%93integral%E2%80%93derivative_controller) is way ahead of us.
So here comes a small primer for
those who never dipped their toes into the muddy waters of control systems.

## The Problem

Defining a control system is hard work. As soon as your system is a bit more
complex than a charging capacitor, you're in for a rough afternoon in the lab
with a lot of measurements, and a sad lonlely weekend of approximations.

If you're not yet done reconsidering your life's carreer choices, you probably
want to go and change something about your system. And as you remember that we
live in the nice days of microcontrollers, you decide to outsource this task of
changing stuff to a controller.

## The Controller

A PID is a mathematical structure that controls stuff. Literally any stuff.
It accepts the control error (the difference between the value you want, and
what you actually have) and reacts to the input.

It consists of 3 almot individual parts:

- P for proportional
- I for integral
- D for differential

### P

Proportional reactions are super simple. It's nothing more than a factor K_p
multiplied to the input error. Say you want to shower at 40°C and your water
supply gives you something like 10°C. Your error is 30K.
If you handle the P controller set to K_p = 100 W/K, you'll set your heaters
output power to 3000W. If you see your water's temperature rise to 35°C, you
change your 500W.

See! Easy math.

### I

The integrator is more challenging as you might have already guessed.
Integrators always are. This time though, the actual implementation
is rather simple as well. Instead of only multiplying the error with a
constant, we multiply it with a constant, and add the result to the result from
before.

Why this? The P channel is fine, but there's a problem. If your control system
doesn't happen to be purely integral itself, a P controller will only
approximate the correct result, but never reach it. Since the I channel
remembers what he did last time someone called it, it can actually reach
the 0 error goal.

Cool! Use it all the time then!

Oh no please not! The I Channel has some weired side effects. It's really
easy to get a system to swing out of control if your I amplification K_i
(sometimes you'll find the inverse T_N = 1/K_i) is not meticulously tuned.

### D

If you thoght you're off the hook, you're wrong. And D is the actual villain.
The D channel also remembers something. It's the last error you've shoved into
PID. D channel takes the difference to the most current error value, and
amplifies the result by a factor K_d (or sometimes called by the inverse T_v).

D Channel can be really helpful to prevent swings, because it resists change.
Ideally you can crank P and I up until everythin is blazingly fast approaching,
but swinging wildly also. All you need is correct your mess by an even more
violent D controller. Right?

Wrong! The D channel is prominently susceptible to noisy inputs. It's a high
pass. So as long as you are dead sure that your input data is clean, a D
channel controller can mess your whole system up. In fact, most control units
I've seen in the wild are disabling the D part because of its volatility.
So make an educated decision about what part of the PID you really want to use.

## Conclusion

The PID is a usefull tool to actually manipulate a system. Make sure you get
the parameters right though. Eventually you're stuck with testing and modelling
on your own.
