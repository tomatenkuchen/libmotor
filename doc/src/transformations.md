# dq-ab-abc transformations

Most likely you can way easier work yourself through this topic by reading
the [Wikipedia](https://de.wikipedia.org/wiki/D/q-Transformation). So go
ahead and take your time.

## Why bother with transformations?

The basic problem that we try to solve with this 3 phase conversion is to
abstract our machine geometry away. Coordinating 3 or more phases on seperate
controllers requires a lot more sinuses, square roots and conversions that we
desire in motor control. Instead of looking at the machine, we want to keep our
eyes on the magnetic field that is produced by the machine. Thankfully this
field is running in principle (and this is where we stay - don't bother about
geometry, 3D magnetic fields, all that realism) in a 2D plane. Physicians think
of vectors, engineers think of complex numbers - who ever you are: you don't
want to stay at a 3 phase perspective looking at a magnetic field. We change
coordinate systems to our good old kartesian friend.

We need a transformation matrix to get there. This was found by Edith Clarke
and does no more than calculating a 3 phase input into a 2D plane. After the
transformation, we have a vector of the magnetic field that our stator
produces. Way more fun!

## Hey, let's transform some more

Engineers are still not happy. There's sinus and cosinus all over the place.
Who in the right mind wants to control that???
Way nicer to handle are constants. Let's have constants. But how?

So far we only focussed on the stator field. Torque in a
[PMDC motor](https://en.wikipedia.org/wiki/Permanent_magnet_motor) is generated
by the interaction of the magnetic fields of the rotor's with the stator's.
Changing to the rotors perspective frees a lot of headspace otherwise
cluttered with phases, angles and amplitudes. Let's see how!

## Park transformation

Assuming the rotor magnet to produce a constant field, we can only control
the stator field with our inverter.
If we emit a magnetic field in the same direction as the rotor's,
we strengthen the magnetic field. They add up. Other way round - weaker field.
If you're not careful, you can even demagnetize the rotor by applying too much
flux in the opposite direction, and ruin the motor.

Sadly though, except for damaging the magnet, or holding the motor in place,
emitting magnetic fields in the rotor's direction doesn't do much. Where's
the rotation coming from that we did all the fuzz for?

Emitting a magnetic vector orthogonal to the rotors magnetic field creates
the torque, that translates over the inertia into rotation. Tadaa! Motor stuff!

I mentioned before: a park transformation or a dq transformation is a rotation.
Nothing more. Nothing fancy. We rotate our newly created 2D model of our stator
magnetic field to align with the rotor's magnetic field. Why?

Because now, all our sinus-cosinus-trigonometry escapes our formulas. By
definition, d-axis lies on the rotor's magnetic direction. the q-axis must
therefore be the orthogonal axis.

If I want more torque, I need more voltage on the q-axis. Positive v_q means
torque in counter-clockwise direction. Negative means, other way around.
Pretty simple, huh?

But wait! There's more! If I need more torque, invest in some positive v_d.
Why? You enhance the rotor's magnetic field (in practice this doesn't happen
that often because your rotor magnet usually is maxed out already).
Want less magnetic field? Give your rotor some negative v_d.
Why would you want to first, pay a lot of money for a really strong magnet
just to ruin my efficiency by using current to weaken it?

In case you want to squeeze some more speed from your motor than it's voltage
actually provides. As long as you're not at torque limit, you can spin faster
with less flux. Mathworks published an nice [explanation](https://www.mathworks.com/discovery/field-weakening-control.html).

## Conclusion

You see, all this transformation is not in vain. Instead of having to deal
with ever swinging voltage curves, we have just 2 very easy levers in our
hands.

- More or less torque --> change v_q.
- More or less flux --> change v_d

So your speed or power controller's output just needs to be v_q or v_d,
you transfrom your voltage back to stator coordinates, back to 3 phase
voltages, and tadaa: motor control!

One drawback: you need to know your rotors position - the rotor angle.
Finding this out is its own can of worms. Let's dig in.
