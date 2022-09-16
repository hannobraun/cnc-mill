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

mod physics;
mod spindle;
mod tools;

use std::fmt;

use physics::Power;

use crate::{physics::Force, spindle::Spindle, tools::Tool};

#[fj::model]
fn cnc() -> fj::Shape {
    let spindle = Spindle::new(Power::from_value_kw(1.5));
    let tools = Tool::tools();

    let (max_force, tool) = tools
        .into_iter()
        .map(|tool| {
            let spindle_torque = spindle.torque(tool.desired_rpm());

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
            let sigma = 483_000_000.; // Pascal

            // The cross-sectional area of the uncut chip depends on axial depth
            // of cut. There's information about that in this document:
            // https://www.sorotec.de/webshop/Datenblaetter/fraeser/schnittwerte.pdf
            //
            // For our calculation, the side milling case is the worst case, due
            // to the higher axial depth of cut.
            let axial_depth_of_cut = tool.length_cutting_edge.value_m();
            let a = axial_depth_of_cut * tool.feed_per_tooth().value_m();

            // For the number of engaged teeth, let's just go with the worst
            // case: At most, the engagement angle is 180°, and the number of
            // engaged teeth is half the total number of teeth.
            let z_c = (tool.num_flutes / 2.).ceil();

            // I don't quite understand what the engagement factor is, but if
            // I'm reading the article right, it's just the radial depth of cut
            // divided by cutting diameter.
            //
            // Radial depth of cut is supposed to be 25% of the cutter diameter
            // for the side milling case we're looking at, according to the
            // Sorotec document linked above.
            let e_f = 0.25;

            // As for cutting tool wear factor, I might be misunderstanding the
            // article, but I think the following should be a good worst case.
            let t_f = 1.6;

            // Now put it all together to calculate the tangential cutting
            // force.
            let tangential_cutting_force =
                Force::from_value_n(sigma * a * z_c * e_f * t_f);

            // Also figure out the torque that would require, and make sure it's
            // below the torque that the spindle can deliver.
            let tool_torque = tangential_cutting_force.to_torque(tool.diameter);
            if tool_torque > spindle_torque {
                println!(
                    "Required torque ({tool_torque}) is larger than spindle \
                    torque ({spindle_torque})!",
                );
                println!("Tool: {tool:#?}");
                println!(
                    "Tangential cutting force: {tangential_cutting_force}"
                );

                return (
                    TangentialCuttingForce::PerMaxSpindleTorque(
                        spindle_torque.to_force(tool.diameter),
                    ),
                    tool,
                );
            }

            (
                TangentialCuttingForce::PerToolRequirements(
                    tangential_cutting_force,
                ),
                tool,
            )
        })
        .reduce(|a, b| if a.0 > b.0 { a } else { b })
        .unwrap();

    println!("Maximum tangential cutting force: {}", max_force);
    println!("Tool: {tool:#?}");

    // This is a placeholder. We don't actually need to export geometry right
    // now, but Fornjot won't allow us to have a function that doesn't do that.
    let w = 0.5;
    fj::Sketch::from_points(vec![[-w, -w], [w, -w], [w, w], [-w, w]]).into()
}

#[derive(Clone, Copy, Debug)]
pub enum TangentialCuttingForce {
    PerToolRequirements(Force),
    PerMaxSpindleTorque(Force),
}

impl TangentialCuttingForce {
    fn value(self) -> Force {
        match self {
            TangentialCuttingForce::PerToolRequirements(value) => value,
            TangentialCuttingForce::PerMaxSpindleTorque(value) => value,
        }
    }
}

impl PartialEq for TangentialCuttingForce {
    fn eq(&self, other: &Self) -> bool {
        self.value().eq(&other.value())
    }
}

impl PartialOrd for TangentialCuttingForce {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

impl fmt::Display for TangentialCuttingForce {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", self.value())?;

        match self {
            TangentialCuttingForce::PerToolRequirements(_) => {
                write!(f, "(per tool requirements)")?
            }
            TangentialCuttingForce::PerMaxSpindleTorque(_) => {
                write!(f, "(limited by max spindle torque)")?
            }
        }

        Ok(())
    }
}
