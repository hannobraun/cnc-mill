use std::collections::BTreeMap;

use crate::physics::{Diameter, Force, Length, RotationalSpeed, Speed};

#[derive(Debug)]
pub struct Tool {
    pub diameter: Diameter,
    pub length_cutting_edge: Length,
    pub length_total: Length,
    pub num_flutes: f64,
}

impl Tool {
    pub fn tools() -> Vec<Self> {
        // This should be a representative selection of tools. I've been trying
        // to find combinations of the smallest diameter and longest length. See
        // research notes.

        macro_rules! tools {
            ($(
                Self {
                    diameter: $diameter:expr,
                    length_cutting_edge: $length_cutting_edge:expr,
                    length_total: $length_total:expr,
                    num_flutes: $num_flutes:expr,
                },
            )*) => {
                vec![
                    $(
                        {
                            let diameter = Diameter::from_length(
                                Length::from_value_mm($diameter),
                            );
                            let length_cutting_edge =
                                Length::from_value_mm($length_cutting_edge);
                            let length_total =
                                Length::from_value_mm($length_total);

                            Self {
                                diameter,
                                length_cutting_edge,
                                length_total,
                                num_flutes: $num_flutes,
                            }
                        },
                    )*
                ]
            };
        }

        tools![
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

    pub fn desired_rpm(&self) -> RotationalSpeed {
        // Cutting speed for aluminium. See this document:
        // https://www.sorotec.de/webshop/Datenblaetter/fraeser/schnittwerte.pdf
        let cutting_speed = Speed::from_value_m_per_min(500.);

        cutting_speed.to_rotational_speed(self.diameter)
    }

    pub fn feed_per_tooth(&self) -> Length {
        // Based on the table on page 2 of this document:
        // https://www.sorotec.de/webshop/Datenblaetter/fraeser/schnittwerte.pdf
        //
        // There are two rows for aluminium. We're choosing the higher values
        // here, since those represent the worst case for our calculation.

        macro_rules! table {
            (
                $(
                    $diameter:expr, $feed_per_tooth_mm:expr;
                )*
            ) => {
                {
                    let mut feed_per_tooth = BTreeMap::new();

                    $(
                        feed_per_tooth.insert($diameter, $feed_per_tooth_mm);
                    )*

                    feed_per_tooth
                }
            };
        }

        let feed_per_tooth = table!(
            1, 0.010;
            2, 0.020;
            3, 0.025;
            4, 0.050;
            5, 0.050;
            6, 0.050;
            8, 0.064;
            10, 0.080;
            12, 0.100;
        );

        let length_mm = *feed_per_tooth
            .get(&(self.diameter.to_length().value_mm().ceil() as u8))
            .unwrap();

        Length::from_value_mm(length_mm)
    }

    pub fn tangential_cutting_force(&self) -> Force {
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
        let axial_depth_of_cut = self.length_cutting_edge.value_m();
        let a = axial_depth_of_cut * self.feed_per_tooth().value_m();

        // For the number of engaged teeth, let's just go with the worst
        // case: At most, the engagement angle is 180°, and the number of
        // engaged teeth is half the total number of teeth.
        let z_c = (self.num_flutes / 2.).ceil();

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
        Force::from_value_n(sigma * a * z_c * e_f * t_f)
    }
}
