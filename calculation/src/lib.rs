//! Calculations for the CNC mill
//!
//! This is basically a Rust program, in the form of a [Fornjot] model, that
//! does some calculations about the CNC mill, to help to select right-sized
//! components.
//!
//! So, why is this a Fornjot model? Honestly, is doesn't make a whole lot of
//! sense, if viewed in isolation. Fornjot doesn't have the features yet to
//! model the CNC mill's geometry, and it has no simulation features at all. It
//! doesn't really support anything I'm using it for here, and I'd probably be
//! better off just doing it in Excel.
//!
//! However, I want to use Fornjot for modeling the CNC mill (or its successor)
//! in the future, and it makes sense to do this kind of calculation together
//! with the CAD model. I also want Fornjot to at least not stand in the way of
//! use cases outside of its (current) core feature that, and using it for
//! something it wasn't designed for should help there.
//!
//! Lastly, I usually only work on Fornjot itself, within its repository. Using
//! it for something new outside of the repository was already hugely
//! informative, in regards to some weaknesses it has, and where it might trip
//! new users up.
//!
//! [Fornjot]: https://www.fornjot.app/

use std::f64::consts::PI;

#[fj::model]
fn cnc() {
    dbg!(torque_nm(5000.));
    dbg!(torque_nm(24000.));

    // This is a placeholder. We don't actually need to export geometry right
    // now, but Fornjot won't allow us to have a function that doesn't do that.
    let w = 0.5;
    fj::Sketch::from_points(vec![[-w, -w], [w, -w], [w, w], [-w, w]]).into()
}

/// Calculate spindle torque in Nm at a given speed in rpm
pub fn torque_nm(rpm: f64) -> f64 {
    let spindle_power_w = 1500.;

    // According to Wikipedia, this is how to calculate power from torque:
    // power = torque * angular speed
    //
    // Hence:
    // torque[Nm] = power[W] / angular speed[rad/s]
    //
    // We got the rotational speed in RPM, so let's convert that to angular
    // speed first.
    let angular_speed = rpm / 60. * 2. * PI;

    // Now we can calculate torque, according to the formula above.
    spindle_power_w / angular_speed
}
