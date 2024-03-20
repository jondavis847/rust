enum Length {
    Au(f64),
    Feet(f64),
    Micrometers(f64),
    Millimeters(f64),
    Centimeters(f64),
    Inches(f64),
    Meters(f64),
    Miles(f64),
    Nanometers(f64),
    NauticalMiles(f64),
    Yards(f64),
}

impl Length {
    fn to_meters(&self) -> f64 {
        match self {
            Length::Au(value) => *value * 1.495978707e+11,
            Length::Feet(value) => *value * 0.3048,
            Length::Micrometers(value) => *value * 1e-6,
            Length::Millimeters(value) => *value * 1e-3,
            Length::Centimeters(value) => *value * 1e-2,
            Length::Inches(value) => *value * 0.0254,
            Length::Kilometers(value) => *value * 1e3,
            Length::Meters(value) => *value,
            Length::Miles(value) => *value * 1609.34,
            Length::Nanometers(value) => *value * 1e-9,
            Length::NauticalMiles(value) => *value * 1852.0,
            Length::Yards(value) => *value * 0.9144,
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
        }
    }

    fn convert(value: f64, l1: Length, l2: Length) -> f64 {
        // Convert value to meters using to_meters method
        let value_in_meters = match l1 {
            Length::Meters => value,
            _ => l1.to_meters(value),
        };

        // Convert value from meters to target units using from_meters method
        match l2 {
            Length::Meters => value_in_meters,
            _ => l2.from_meters(value_in_meters),
        }
    }
}

fn main() {
    // Convert 10 feet to meters
    let value_in_meters = Length::convert(10.0, Length::Feet, Length::Meters);
    println!("10 feet is {} meters", value_in_meters);

    // Convert 1000 meters to kilometers
    let value_in_kilometers = Length::convert(1000.0, Length::Meters, Length::Kilometers);
    println!("1000 meters is {} kilometers", value_in_kilometers);
}
