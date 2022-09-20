use std::{
    f64::consts::{PI, TAU},
    fmt,
    ops::{Add, Div},
};

/// A diameter
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Diameter(Length);

impl Diameter {
    /// Create an instance of `Diameter` from a `Length`
    pub const fn from_length(length: Length) -> Self {
        Self(length)
    }

    /// Convert this diameter into a `Length`
    pub fn to_length(&self) -> Length {
        self.0
    }

    /// Convert this diameter into a `Radius`
    pub fn to_radius(&self) -> Radius {
        Radius::from_length(self.to_length() / 2.)
    }
}

/// A force
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Force(f64);

impl Force {
    /// Create an instance of `Force` from a value in Newton
    pub const fn from_value_n(force_n: f64) -> Self {
        Self(force_n)
    }

    /// Return the value in Newton
    pub fn value_n(&self) -> f64 {
        self.0
    }

    /// Compute the torque resulting from this force at the given radius
    pub fn to_torque(&self, radius: impl Into<Radius>) -> Torque {
        let torque_nm = self.value_n() * radius.into().to_length().value_m();
        Torque::from_value_nm(torque_nm)
    }
}

impl fmt::Display for Force {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} N", self.value_n())
    }
}

/// A length
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Length(f64);

impl Length {
    /// Create an instance of `Length` from a value in meter
    pub const fn from_value_m(length_m: f64) -> Self {
        Self(length_m)
    }

    /// Create an instance of `Length` from a value in millimeter
    pub fn from_value_mm(length_mm: f64) -> Self {
        Self::from_value_m(length_mm / 1000.)
    }

    /// Return the value in meter
    pub fn value_m(&self) -> f64 {
        self.0
    }

    /// Return the value in millimeter
    pub fn value_mm(&self) -> f64 {
        self.0 * 1000.
    }
}

impl Add<Self> for Length {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Div<f64> for Length {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

/// A power value
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Power(f64);

impl Power {
    pub const fn from_value_w(power_w: f64) -> Self {
        Self(power_w)
    }

    pub fn from_value_kw(power_kw: f64) -> Self {
        Self::from_value_w(power_kw * 1000.)
    }

    pub fn value_w(&self) -> f64 {
        self.0
    }

    pub fn to_torque(&self, rotational_speed: RotationalSpeed) -> Torque {
        Torque::from_value_nm(
            self.value_w() / rotational_speed.value_rad_per_s(),
        )
    }
}

/// A radius
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Radius(Length);

impl Radius {
    /// Create an instance of `Radius` from a `Length`
    pub const fn from_length(length: Length) -> Self {
        Self(length)
    }

    /// Convert this radius into a `Length`
    pub fn to_length(&self) -> Length {
        self.0
    }
}

impl From<Diameter> for Radius {
    fn from(diameter: Diameter) -> Self {
        diameter.to_radius()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct RotationalSpeed(f64);

impl RotationalSpeed {
    pub const fn from_value_rpm(rotational_speed_rpm: f64) -> Self {
        Self(rotational_speed_rpm)
    }

    pub fn value_rpm(&self) -> f64 {
        self.0
    }

    pub fn value_rad_per_s(&self) -> f64 {
        self.value_rpm() / 60. * 2. * PI
    }

    pub fn clamp(&self, min: RotationalSpeed, max: RotationalSpeed) -> Self {
        Self(self.0.clamp(min.0, max.0))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Speed(f64);

impl Speed {
    pub const fn from_value_m_per_s(speed_m_per_s: f64) -> Self {
        Self(speed_m_per_s)
    }

    pub fn from_value_m_per_min(speed_m_per_min: f64) -> Self {
        Self::from_value_m_per_s(speed_m_per_min / 60.)
    }

    pub fn value_m_per_min(&self) -> f64 {
        self.0 * 60.
    }

    pub fn to_rotational_speed(
        &self,
        radius: impl Into<Radius>,
    ) -> RotationalSpeed {
        RotationalSpeed::from_value_rpm(
            self.value_m_per_min() / radius.into().to_length().value_m() / TAU,
        )
    }
}

/// A torque
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Torque(f64);

impl Torque {
    /// Create an instance of `Torque` from a value in Nm
    pub const fn from_value_nm(torque_nm: f64) -> Self {
        Self(torque_nm)
    }

    /// Return the value in Nm
    pub fn value_nm(&self) -> f64 {
        self.0
    }

    /// Compute the force resulting from this torque at the given radius
    pub fn to_force(&self, radius: impl Into<Radius>) -> Force {
        let force_n = self.value_nm() / radius.into().to_length().value_m();
        Force::from_value_n(force_n)
    }
}

impl fmt::Display for Torque {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} Nm", self.value_nm())
    }
}
