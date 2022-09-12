use std::ops::Div;

/// A diameter
#[derive(Clone, Copy, Debug)]
pub struct Diameter(Length);

impl Diameter {
    /// Create an instance of `Diameter` from a `Length`
    pub fn from_length(length: Length) -> Self {
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
#[derive(Debug)]
pub struct Force(f64);

impl Force {
    /// Create an instance of `Force` from a value in Newton
    pub fn from_value_n(force_n: f64) -> Self {
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

/// A length
#[derive(Clone, Copy, Debug)]
pub struct Length(f64);

impl Length {
    /// Create an instance of `Length` from a value in meter
    pub fn from_value_m(length_m: f64) -> Self {
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

impl Div<f64> for Length {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

/// A radius
#[derive(Clone, Copy)]
pub struct Radius(Length);

impl Radius {
    /// Create an instance of `Radius` from a `Length`
    pub fn from_length(length: Length) -> Self {
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

/// A torque
pub struct Torque(f64);

impl Torque {
    /// Create an instance of `Torque` from a value in Nm
    pub fn from_value_nm(torque_nm: f64) -> Self {
        Self(torque_nm)
    }

    /// Return the value in Nm
    pub fn value_nm(&self) -> f64 {
        self.0
    }
}
