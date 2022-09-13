use crate::physics::{Power, RotationalSpeed, Torque};

pub struct Spindle {
    power: Power,
}

impl Spindle {
    const MIN: RotationalSpeed = RotationalSpeed::from_value_rpm(5000.);
    const MAX: RotationalSpeed = RotationalSpeed::from_value_rpm(24000.);

    pub fn new(power: Power) -> Self {
        Self { power }
    }

    /// Calculate spindle torque in Nm at a given speed in rpm
    pub fn torque(&self, rotational_speed: RotationalSpeed) -> Torque {
        let rotational_speed = rotational_speed.clamp(Self::MIN, Self::MAX);
        self.power.to_torque(rotational_speed)
    }
}
