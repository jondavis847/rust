pub enum Angle {
    Degrees,
    Radians,
    Arcseconds,
}

impl Angle {
    fn to_radians(&self, value: f64) -> f64 {
        match self {
            Angle::Degrees => value.to_radians(),
            Angle::Radians => value,
            Angle::Arcseconds => value.to_radians() / 3600.0,
        }
    }

    fn from_radians(&self, value: f64) -> f64 {
        match self {
            Angle::Degrees => value.to_degrees(),
            Angle::Radians => value,
            Angle::Arcseconds => value.to_degrees() * 3600.0,
        }
    }

    pub fn convert(value: f64, l1: Angle, l2: Angle) -> f64 {
        let value_in_radians = l1.to_radians(value);
        l2.from_radians(value_in_radians)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_radians() {
        assert_eq!(Angle::Degrees.to_radians(180.0), std::f64::consts::PI);
        assert_eq!(Angle::Radians.to_radians(std::f64::consts::PI), std::f64::consts::PI);
        assert_eq!(Angle::Arcseconds.to_radians(3600.0), std::f64::consts::PI / 180.0);
    }

    #[test]
    fn test_from_radians() {
        assert_eq!(Angle::Degrees.from_radians(std::f64::consts::PI), 180.0);
        assert_eq!(Angle::Radians.from_radians(std::f64::consts::PI), std::f64::consts::PI);
        assert_eq!(Angle::Arcseconds.from_radians(std::f64::consts::PI / 180.0), 3600.0);
    }

    #[test]
    fn test_convert() {
        // Conversion from each unit to every other unit
        assert_eq!(
            Angle::convert(std::f64::consts::PI, Angle::Radians, Angle::Degrees),
            180.0
        );
        assert_eq!(
            Angle::convert(180.0, Angle::Degrees, Angle::Radians),
            std::f64::consts::PI
        );

        assert_eq!(
            Angle::convert(std::f64::consts::PI / 180.0, Angle::Radians, Angle::Arcseconds),
            3600.0
        );
        assert_eq!(
            Angle::convert(3600.0, Angle::Arcseconds, Angle::Radians),
            std::f64::consts::PI / 180.0
        );

        // Additional conversion tests
        assert_eq!(
            Angle::convert(90.0, Angle::Degrees, Angle::Arcseconds),
            324000.0
        );
        assert_eq!(
            Angle::convert(0.1, Angle::Arcseconds, Angle::Degrees),
            2.7777777777777782e-5
        );
    }
}
