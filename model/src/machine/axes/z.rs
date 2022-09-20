use crate::physics::Length;

/// The max travel of the z-axis
///
/// This should be a constant, but it can't as floating-point arithmetic can not
/// be `const`.
pub fn max_travel() -> Length {
    Length::from_value_mm(100.)
}
