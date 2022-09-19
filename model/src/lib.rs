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

mod machine;
mod physics;
mod tools;

use std::fmt;

use physics::Power;

use crate::{machine::spindle::Spindle, physics::Force, tools::Tool};

#[fj::model]
fn cnc() -> fj::Shape {
    let spindle = Spindle::new(Power::from_value_kw(1.5));
    let tools = Tool::tools();

    let (max_force, tool) = tools
        .into_iter()
        .map(|tool| {
            let (tangential_cutting_force, tool_torque) =
                tool.tangential_cutting_force();

            // Also figure out the torque that would require, and make sure it's
            // below the torque that the spindle can deliver.
            let spindle_torque = spindle.torque(tool.desired_rpm());
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
