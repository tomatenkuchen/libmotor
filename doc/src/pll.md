# PLLs

## Intro

PLL stands for phase locked loop. If you're already familiar with control systems
you sniff control loops already in the name. And you're right. Good dog!

### Phase

Deviations from a periodic event are defined by their phase. That's our first
clue whats going on here.

### Locked

Someone wants to keep the phase locked somewhere. Hmmm... how could that be possible?

### Loop

Aaaaah there we are. We are in a control loop. So what is a PLL now?

### 1 + 1 + 1

A PLL is a special form of control loop. Instead of temperature or position, we
control a phase. This is a fancy way of using a PI controller to keep the
position of something in place.

But how does this help in motor control?

## What the Phase?

We already established in our [transformation](./transformations.md) section
that knowledge on our rotor is vital for our motor controller to properly work.
Sadly, without a sensor attached, the motor doesn't simply tell us what state it
is in. We need to figure out by ourselves.

One way to calculate this, is taking the 3 current values we get from our
inverter, and try to calculate the induction voltage.

Todo
