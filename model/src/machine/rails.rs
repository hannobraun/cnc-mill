use crate::physics::Length;

/// Let's just assume MGN15 for now, unless it turns out not to be sufficient.
pub fn mgn15_height_total() -> Length {
    // See HIWIN catalogue table 3.79 on page 97/
    Length::from_value_mm(16.)
}
