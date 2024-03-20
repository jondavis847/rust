use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct UncertainValue<T> {
    pub value: T,
    pub uncertainty: T,
}

/// A value with uncertainty
impl<T> UncertainValue<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + std::ops::AddAssign
        + std::ops::MulAssign
        + Copy
        + PartialOrd
        + std::ops::Neg<Output = T>,
{
    /// Constructor for creating an UncertainValue
    pub fn new(value: T, uncertainty: T) -> Self {
        Self { value, uncertainty }
    }
}

// Addition operation for UncertainValues
impl<T> Add for UncertainValue<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
            uncertainty: self.uncertainty + other.uncertainty,
        }
    }
}

/// Subtraction operation for UncertainValues
impl<T> Sub for UncertainValue<T>
where
    T: std::ops::Sub<Output = T> + std::ops::Add<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            value: self.value - other.value,
            uncertainty: self.uncertainty + other.uncertainty,
        }
    }
}

/// Multiplication operation for UncertainValues
impl<T> Mul for UncertainValue<T>
where
    T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let new_value = self.value * other.value;
        let new_uncertainty = self.uncertainty * other.value + other.uncertainty * self.value;
        Self {
            value: new_value,
            uncertainty: new_uncertainty,
        }
    }
}

/// Division operation for UncertainValues
impl<T> Div for UncertainValue<T>
where
    T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let new_value = self.value / other.value;
        let new_uncertainty = (self.uncertainty / other.value)
            + (self.value * other.uncertainty) / (other.value * other.value);
        Self {
            value: new_value,
            uncertainty: new_uncertainty,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_f32() {
        let a = UncertainValue::<f32>::new(5.0, 0.1);
        let b = UncertainValue::<f32>::new(3.0, 0.05);
        let result = a + b;
        assert_eq!(result.value, 8.0);
        assert_eq!(result.uncertainty, 0.15000000000000002); //Adjust for floating point specification
    }

    #[test]
    fn test_sub_f32() {
        let a = UncertainValue::<f32>::new(5.0, 0.1);
        let b = UncertainValue::<f32>::new(3.0, 0.05);
        let result = a - b;
        assert_eq!(result.value, 2.0);
        assert_eq!(result.uncertainty, 0.15000000000000002); //Adjust for floating point specification
    }

    #[test]
    fn test_mul_f32() {
        let a = UncertainValue::<f32>::new(5.0, 0.1);
        let b = UncertainValue::<f32>::new(3.0, 0.05);
        let result = a * b;
        assert_eq!(result.value, 15.0);
        assert_eq!(result.uncertainty, 0.55);
    }

    #[test]
    fn test_div_f32() {
        let a = UncertainValue::<f32>::new(5.0, 0.1);
        let b = UncertainValue::<f32>::new(2.0, 0.05);
        let result = a / b;
        assert_eq!(result.value, 2.5);
        assert_eq!(result.uncertainty, 0.1125);
    }

    #[test]
    fn test_add_f64() {
        let a = UncertainValue::<f64>::new(5.0, 0.1);
        let b = UncertainValue::<f64>::new(3.0, 0.05);
        let result = a + b;
        assert_eq!(result.value, 8.0);
        assert_eq!(result.uncertainty, 0.15000000000000002);
    }

    #[test]
    fn test_sub_f64() {
        let a = UncertainValue::<f64>::new(5.0, 0.1);
        let b = UncertainValue::<f64>::new(3.0, 0.05);
        let result = a - b;
        assert_eq!(result.value, 2.0);
        assert_eq!(result.uncertainty, 0.15000000000000002); //Adjust for floating point specification
    }

    #[test]
    fn test_mul_f64() {
        let a = UncertainValue::<f64>::new(5.0, 0.1);
        let b = UncertainValue::<f64>::new(3.0, 0.05);
        let result = a * b;
        assert_eq!(result.value, 15.0);
        assert_eq!(result.uncertainty, 0.55);
    }

    #[test]
    fn test_div_f64() {
        let a = UncertainValue::<f64>::new(5.0, 0.1);
        let b = UncertainValue::<f64>::new(2.0, 0.05);
        let result = a / b;
        assert_eq!(result.value, 2.5);
        assert_eq!(result.uncertainty, 0.1125);
    }
}
