/// A force
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
}

/// A length
#[derive(Debug)]
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
}
