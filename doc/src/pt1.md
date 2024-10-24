# PT1 filter

## Intro

As seen before: here's the [Wikipedia PT1](https://de.wikipedia.org/wiki/PT1-Glied)
that should get it all out there. I don't know why there's no english
article about it, but you have translators I suppose.

PT1 is a standard unit of delay, which itself is build of an integrator
and a negative feedback loop from a proportional unit. It's characteristics
are defined by its amplification K_p and its time constant T. The higher the
T, the stronger the low pass filter.

In the Wiki article you'll find a lot of Euler curves. These would completely
defeat the purpose of this library: having something less compute intensive.

## Moving Averages

The good part is: knowing of it's components, we can build our PT1 with only
2 building blocks: integrator and proportional unit.

todo: here comes a chart of the control circuit

## Conclusion

With only basic arithmetic, we can create a low pass filter with well defined
time characteristics.
