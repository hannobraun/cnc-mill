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

use std::{collections::BTreeMap, f64::consts::PI};

#[fj::model]
fn cnc() -> fj::Shape {
    let spindle = Spindle::new(W(1500.));
    let tools = Tool::tools();

    let max_force_n = tools
        .into_iter()
        .map(|tool| {
            let rpm = tool.desired_rpm();
            let torque = spindle.torque(rpm);
            let tool_radius_m = tool.diameter / 2. / 1000.;

            // This article talks about tangential cutting force:
            // https://www.ctemag.com/news/articles/understanding-tangential-cutting-force-when-milling
            //
            // It gives the following formula; (2) in the article:
            // Ft = sigma * A * Zc * Ef * Tf
            //
            // Ft: tangential cutting force
            // sigma: ultimate tensile strength (σ)
            // A: cross-sectional area of the uncut chip
            // Zc: number of teeth engaged in workpiece
            // Ef: engagement factor of workpiece material
            // Tf: cutting tool wear factor
            //
            // Wikipedia has an article on ultimate tensile strength:
            // https://en.wikipedia.org/wiki/Ultimate_tensile_strength
            //
            // According to the table in there, this is the value for aluminium:
            let sigma = 483.; // MPa
            dbg!(&sigma);

            // The cross-sectional area of the uncut chip depends on axial depth
            // of cut. There's information about that in this document:
            // https://www.sorotec.de/webshop/Datenblaetter/fraeser/schnittwerte.pdf
            //
            // For our calculation, the side milling case is the worst case, due
            // to the higher axial depth of cut.
            let axial_depth_of_cut = tool.length_cutting_edge;
            let a = axial_depth_of_cut * tool.feed_per_tooth();
            dbg!(&a);

            // For the number of engaged teeth, let's just go with the worst
            // case: At most, the engagement angle is 180°, and the number of
            // engaged teeth is half the total number of teeth.
            let z_c = (tool.num_flutes / 2.).ceil();
            dbg!(z_c);

            // I don't quite understand what the engagement factor is, but if
            // I'm reading the article right, it's just the radial depth of cut
            // divided by cutting diameter.
            //
            // Radial depth of cut is supposed to be 25% of the cutter diameter
            // for the side milling case we're looking at, according to the
            // Sorotec document linked above.
            let e_f = 0.25;
            dbg!(&e_f);

            // TASK: Calculate the rest of those quantities.

            // We now have the spindle torque for the given tool at its desired
            // RPM. Based on that, we calculate the absolute worst-case
            // tangential cutting force.
            //
            // This is not going to be a realistic value. It doesn't take depth
            // of cut into account, or really anything else. It's the worst
            // possible force that the calculated torque could result in.
            //
            // I'm not completely sure if this calculation is even right, but it
            // certainly needs to be refined, to come up with a useful value.
            torque.0 / tool_radius_m
        })
        .reduce(f64::max);

    dbg!(max_force_n);

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
        let rpm = rpm.0.min(Self::MAX_RPM.0).max(Self::MIN_RPM.0);

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

    pub fn desired_rpm(&self) -> Rpm {
        // Cutting speed for aluminium. See this document:
        // https://www.sorotec.de/webshop/Datenblaetter/fraeser/schnittwerte.pdf
        let cutting_speed = MperM(500.);

        // Formula for calculating spindle RPM. See same document.
        Rpm(cutting_speed.0 * 1000. / self.diameter / PI)
    }

    pub fn feed_per_tooth(&self) -> f64 {
        // Based on the table on page 2 of this document:
        // https://www.sorotec.de/webshop/Datenblaetter/fraeser/schnittwerte.pdf
        //
        // There are two rows for aluminium. We're choosing the higher values
        // here, since those represent the worst case for our calculation.

        let mut feed_per_tooth = BTreeMap::new();
        feed_per_tooth.insert(1, 0.010);
        feed_per_tooth.insert(2, 0.020);
        feed_per_tooth.insert(3, 0.025);
        feed_per_tooth.insert(4, 0.050);
        feed_per_tooth.insert(5, 0.050);
        feed_per_tooth.insert(6, 0.050);
        feed_per_tooth.insert(8, 0.064);
        feed_per_tooth.insert(10, 0.080);
        feed_per_tooth.insert(12, 0.100);

        feed_per_tooth.get(&(self.diameter.ceil() as u8)).unwrap() / 1000.
    }
}

pub struct W(pub f64);

pub struct Rpm(pub f64);

#[derive(Debug)]
pub struct Nm(pub f64);

/// Meter per minute
pub struct MperM(pub f64);
