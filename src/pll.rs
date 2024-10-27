#![deny(unsafe_code)]
#![deny(missing_docs)]

//! PLL module. This function in here serves the purpose of sensorless motor state estimation from
//! electrical data. So far it's scope is limited to passive voltage and current sensing. More
//! clealy, it excludes [high frequency
//! injection](https://ieeexplore.ieee.org/abstract/document/5316521).
//!
//! One of your best hints from electrical data we have about the rotor's speed and position is
//! it's induced voltage. It is emitted orthogonal to the rotor's flux axis (usually the d axis).
//! Therefore, if we can calculate V_ind from V_1 and I, we have quite a good guess on how fast the
//! rotor is.
//!
//! But wait, you'll say. Even if I make it to have a mediocre measure of speed
//! 1 .how on earth should that work from the start? No speed - no V_ind
//! 2. what about rotor position?  You can't really measure that, can you?
//!
//! Good points! To point 1: Correct. You can use this method only if the motor is already
//! spinning. Either you will use this fancy magic of high frequency injection mentioned
//! above, which has some crazy logic to it and a couple of other drawbacks, or you just wing it.
//! In most cases, where your main operational point is not super slow paced, or correct rotor
//! position stop is not a necessity, you'll simply open-loop muscel the rotor up to speed with bad
//! efficiency and really bad acoustics, but as soon as you're up to speed you activate the
//! algorithm and start your normal control path.
//!
//! Second problem is - as you've guessed - already solved, but not in this straight foreward way
//! you'd wish.  On the other side it has given this modules name. We use a PID control block, feed
//! it with the dq-transformed V_ind you calculated previously - esprecially the d part. Your V_ind
//! should not have a real ingredient, so whenever you see one, you change your output speed
//! accordingly. Your rotor's position is then only a matter of integration over time.
//! Is that accurately measured? No! But don't bother. Since your output voltage uses your
//! calculated rotor speed and position, your motor keeps itself controlled. Of course, the more
//! accurate your speed and rotor position, the better your efficiency. In most cases you'll be ok.
//! Hopefully!
//!

use crate::dq::abc2ab;
use crate::motor::Mechanical;

/// extract rotor state information from electrial data
/// TODO: check if it's a good idea to consume mutabl borrow of motor state instead
pub fn pll(v_1: [f32; 3], i: [f32; 3]) -> Mechanical {
    // transform input to ab
    // calc V_z
    // subtract V_z from V_1 to get V_ind
    // feed pll with V_ind.re to create feedback loop
    // integrate rotor speed for angle
}
