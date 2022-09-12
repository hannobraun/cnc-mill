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
