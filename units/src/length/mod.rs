pub enum Length {
    Au,
    Centimeters,
    Feet,
    Inches,
    Kilometers,
    Meters,
    Micrometers,
    Millimeters,
    Miles,
    Nanometers,
    NauticalMiles,
    Yards,
    Parsecs,
    LightYears,
}

impl Length {
    fn to_meters(&self, value: f64) -> f64 {
        match self {
            Length::Au => value * 1.495978707e+11,
            Length::Feet => value * 0.3048,
            Length::Micrometers => value * 1e-6,
            Length::Millimeters => value * 1e-3,
            Length::Centimeters => value * 1e-2,
            Length::Inches => value * 0.0254,
            Length::Kilometers => value * 1e3,
            Length::Meters => value,
            Length::Miles => value * 1609.34,
            Length::Nanometers => value * 1e-9,
            Length::NauticalMiles => value * 1852.0,
            Length::Yards => value * 0.9144,
            Length::Parsecs => value * 3.08567758e16, // 1 parsec = 3.08567758 × 10^16 meters
            Length::LightYears => value * 9.461e15,   // 1 light year = 9.461 × 10^15 meters
        }
    }

    fn from_meters(&self, value: f64) -> f64 {
        match self {
            Length::Au => value / 1.495978707e+11,
            Length::Feet => value / 0.3048,
            Length::Micrometers => value / 1e-6,
            Length::Millimeters => value / 1e-3,
            Length::Centimeters => value / 1e-2,
            Length::Inches => value / 0.0254,
            Length::Kilometers => value / 1e3,
            Length::Meters => value,
            Length::Miles => value / 1609.34,
            Length::Nanometers => value / 1e-9,
            Length::NauticalMiles => value / 1852.0,
            Length::Yards => value / 0.9144,
            Length::Parsecs => value / 3.08567758e16,
            Length::LightYears => value / 9.461e15,
        }
    }

    pub fn convert(value: f64, l1: Length, l2: Length) -> f64 {
        let value_in_meters = l1.to_meters(value);
        l2.from_meters(value_in_meters)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_meters() {
        assert_eq!(Length::Au.to_meters(1.0), 1.495978707e11);
        assert_eq!(Length::Feet.to_meters(1.0), 0.3048);
        assert_eq!(Length::Micrometers.to_meters(1.0), 1e-6);
        assert_eq!(Length::Millimeters.to_meters(1.0), 1e-3);
        assert_eq!(Length::Centimeters.to_meters(1.0), 1e-2);
        assert_eq!(Length::Inches.to_meters(1.0), 0.0254);
        assert_eq!(Length::Kilometers.to_meters(1.0), 1e3);
        assert_eq!(Length::Meters.to_meters(1.0), 1.0);
        assert_eq!(Length::Miles.to_meters(1.0), 1609.34);
        assert_eq!(Length::Nanometers.to_meters(1.0), 1e-9);
        assert_eq!(Length::NauticalMiles.to_meters(1.0), 1852.0);
        assert_eq!(Length::Yards.to_meters(1.0), 0.9144);
        assert_eq!(Length::Parsecs.to_meters(1.0), 3.08567758e16);
        assert_eq!(Length::LightYears.to_meters(1.0), 9.461e15);
    }

    #[test]
    fn test_from_meters() {
        assert_eq!(Length::Au.from_meters(1.495978707e11), 1.0);
        assert_eq!(Length::Feet.from_meters(0.3048), 1.0);
        assert_eq!(Length::Micrometers.from_meters(1e-6), 1.0);
        assert_eq!(Length::Millimeters.from_meters(1e-3), 1.0);
        assert_eq!(Length::Centimeters.from_meters(1e-2), 1.0);
        assert_eq!(Length::Inches.from_meters(0.0254), 1.0);
        assert_eq!(Length::Kilometers.from_meters(1e3), 1.0);
        assert_eq!(Length::Meters.from_meters(1.0), 1.0);
        assert_eq!(Length::Miles.from_meters(1609.34), 1.0);
        assert_eq!(Length::Nanometers.from_meters(1e-9), 1.0);
        assert_eq!(Length::NauticalMiles.from_meters(1852.0), 1.0);
        assert_eq!(Length::Yards.from_meters(0.9144), 1.0);
        assert_eq!(Length::Parsecs.from_meters(3.08567758e16), 1.0);
        assert_eq!(Length::LightYears.from_meters(9.461e15), 1.0);
    }

    #[test]
    fn test_convert() {
        // Conversion from each unit to every other unit
        assert_eq!(
            Length::convert(1.0, Length::Au, Length::Feet),
            490806662401.57477
        );
        assert_eq!(
            Length::convert(1.0, Length::Feet, Length::Au),
            2.0374621548674225e-12
        );

        assert_eq!(
            Length::convert(1.0, Length::Micrometers, Length::Millimeters),
            0.001
        );
        assert_eq!(
            Length::convert(1.0, Length::Millimeters, Length::Micrometers),
            1000.0000000000001 //floating point specification
        );

        assert_eq!(
            Length::convert(1.0, Length::Centimeters, Length::Inches),
            0.3937007874015748
        );
        assert_eq!(
            Length::convert(1.0, Length::Inches, Length::Centimeters),
            2.54
        );

        assert_eq!(
            Length::convert(1.0, Length::Kilometers, Length::Miles),
            0.6213727366498067
        );
        assert_eq!(
            Length::convert(1.0, Length::Miles, Length::Kilometers),
            1.60934
        );

        assert_eq!(
            Length::convert(1.0, Length::Nanometers, Length::NauticalMiles),
            5.399568034557235e-13
        );
        assert_eq!(
            Length::convert(1.0, Length::NauticalMiles, Length::Nanometers),
            1.852e12
        );

        assert_eq!(
            Length::convert(1.0, Length::Yards, Length::Parsecs),
            2.9633685837001804e-17
        );
        assert_eq!(
            Length::convert(1.0, Length::Parsecs, Length::Yards),
            3.3745380358705164e16
        );

        assert_eq!(
            Length::convert(1.0, Length::LightYears, Length::Centimeters),
            9.461e17
        );
        assert_eq!(
            Length::convert(1.0, Length::Centimeters, Length::LightYears),
            1.056970721911003e-18
        );
    }
}
