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
fn cnc() -> fj::Shape {
    let spindle = Spindle::new(W(1500.));

    dbg!(spindle.torque(Spindle::MIN_RPM));
    dbg!(spindle.torque(Spindle::MAX_RPM));

    // TASK: I can test every tool at min and max RPM, but does that make sense?
    //       Small diameter tools will apply the largest force for a given
    //       moment, but they will have smaller moments due to higher RPM.

    // This is a placeholder. We don't actually need to export geometry right
    // now, but Fornjot won't allow us to have a function that doesn't do that.
    let w = 0.5;
    fj::Sketch::from_points(vec![[-w, -w], [w, -w], [w, w], [-w, w]]).into()
}

pub struct Spindle {
    power: W,
}

impl Spindle {
    const MIN_RPM: Rpm = Rpm(5000.);
    const MAX_RPM: Rpm = Rpm(24000.);

    pub fn new(power: W) -> Self {
        Self { power }
    }

    /// Calculate spindle torque in Nm at a given speed in rpm
    pub fn torque(&self, rpm: Rpm) -> Nm {
        // According to Wikipedia, this is how to calculate power from torque:
        // power = torque * angular speed
        //
        // Hence:
        // torque[Nm] = power[W] / angular speed[rad/s]
        //
        // We got the rotational speed in RPM, so let's convert that to angular
        // speed first.
        let angular_speed = rpm.0 / 60. * 2. * PI;

        // Now we can calculate torque, according to the formula above.
        Nm(self.power.0 / angular_speed)
    }
}

pub struct Tool {
    pub diameter: f64,
    pub length_cutting_edge: f64,
    pub length_total: f64,
    pub num_flutes: f64,
}

impl Tool {
    pub fn tools() -> Vec<Self> {
        // This should be a representative selection of tools. I've been trying
        // to find combinations of the smallest diameter and longest length. See
        // research notes.

        vec![
            // https://www.sorotec.de/shop/Zerspanungswerkzeuge/sorotec-werkzeuge/1-8-werkzeuge/3-175----1-8---Fraeser/2-Schneider-ALU/
            Self {
                diameter: 0.4,
                length_cutting_edge: 2.0,
                length_total: 38.0,
                num_flutes: 2.,
            },
            Self {
                diameter: 0.5,
                length_cutting_edge: 2.5,
                length_total: 38.0,
                num_flutes: 2.,
            },
            Self {
                diameter: 0.6,
                length_cutting_edge: 3.0,
                length_total: 38.0,
                num_flutes: 2.,
            },
            Self {
                diameter: 1.5,
                length_cutting_edge: 12.0,
                length_total: 38.0,
                num_flutes: 2.,
            },
            Self {
                diameter: 1.6,
                length_cutting_edge: 5.0,
                length_total: 38.0,
                num_flutes: 2.,
            },
            Self {
                diameter: 1.8,
                length_cutting_edge: 6.0,
                length_total: 38.0,
                num_flutes: 2.,
            },
            Self {
                diameter: 2.0,
                length_cutting_edge: 12.0,
                length_total: 38.0,
                num_flutes: 2.,
            },
            Self {
                diameter: 2.4,
                length_cutting_edge: 7.0,
                length_total: 38.0,
                num_flutes: 2.,
            },
            Self {
                diameter: 2.5,
                length_cutting_edge: 15.0,
                length_total: 38.0,
                num_flutes: 2.,
            },
            Self {
                diameter: 3.175,
                length_cutting_edge: 5.0,
                length_total: 8.0,
                num_flutes: 2.,
            },
            // https://www.sorotec.de/shop/Zerspanungswerkzeuge/sorotec-werkzeuge/1-schneider/Schaftfraeser-ALU-412/
            Self {
                diameter: 3.0,
                length_cutting_edge: 22.0,
                length_total: 50.0,
                num_flutes: 1.,
            },
            Self {
                diameter: 6.0,
                length_cutting_edge: 26.0,
                length_total: 68.0,
                num_flutes: 1.,
            },
            Self {
                diameter: 6.0,
                length_cutting_edge: 21.0,
                length_total: 80.0,
                num_flutes: 1.,
            },
            Self {
                diameter: 10.0,
                length_cutting_edge: 26.0,
                length_total: 110.0,
                num_flutes: 1.,
            },
            // https://www.sorotec.de/shop/Zerspanungswerkzeuge/sorotec-werkzeuge/1-schneider/1-Schneider-Sorotec-PROALU/
            Self {
                diameter: 2.0,
                length_cutting_edge: 8.0,
                length_total: 50.0,
                num_flutes: 1.,
            },
            // https://www.sorotec.de/shop/Zerspanungswerkzeuge/sorotec-werkzeuge/1-schneider/einschneider-sorotec-alu-beschichtet/
            Self {
                diameter: 1.0,
                length_cutting_edge: 5.0,
                length_total: 40.0,
                num_flutes: 1.,
            },
            Self {
                diameter: 2.0,
                length_cutting_edge: 5.0,
                length_total: 40.0,
                num_flutes: 1.,
            },
            // https://www.sorotec.de/shop/Zerspanungswerkzeuge/sorotec-werkzeuge/2-schneider/Schaftfraeser-ALU/
            Self {
                diameter: 4.0,
                length_cutting_edge: 21.0,
                length_total: 70.0,
                num_flutes: 2.,
            },
            Self {
                diameter: 5.0,
                length_cutting_edge: 30.0,
                length_total: 75.0,
                num_flutes: 2.,
            },
            Self {
                diameter: 6.0,
                length_cutting_edge: 30.0,
                length_total: 75.0,
                num_flutes: 2.,
            },
            Self {
                diameter: 8.0,
                length_cutting_edge: 40.0,
                length_total: 100.0,
                num_flutes: 2.,
            },
            Self {
                diameter: 10.0,
                length_cutting_edge: 40.0,
                length_total: 100.0,
                num_flutes: 2.,
            },
            Self {
                diameter: 12.0,
                length_cutting_edge: 32.0,
                length_total: 74.0,
                num_flutes: 2.,
            },
            // https://www.sorotec.de/shop/Zerspanungswerkzeuge/sorotec-werkzeuge/RADIENFRAeSER/1-Schneider-PRO/
            Self {
                diameter: 2.0,
                length_cutting_edge: 6.0,
                length_total: 39.0,
                num_flutes: 1.,
            },
        ]
    }
}

pub struct W(pub f64);

pub struct Rpm(pub f64);

#[derive(Debug)]
pub struct Nm(pub f64);
